extern crate metaflac;

use walkdir::WalkDir;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use chrono::naive::NaiveDate;
use chrono::Datelike;
use metaflac::block::PictureType as FLACPictureType;
use id3::frame::PictureType as ID3PictureType;

use crate::traxsource::Track;
use crate::tagger::TaggerConfig;

//Get and parse tags from all supported files from all subdirectories
pub fn load_files(path: &str) -> Vec<AudioFileInfo> {
    let supported_extensions = vec![".mp3", ".flac", ".aif", ".aiff"];

    WalkDir::new(path).into_iter().filter(
        //Valid files
        |e| e.is_ok() &&
        //Extensions
        supported_extensions.iter().any(|i| e.as_ref().unwrap().path().to_str().unwrap().to_ascii_lowercase().ends_with(i))
    ).map(|e| e.unwrap().path().to_str().unwrap().to_owned())
    //Load info
    .filter_map(|f| {
        match load_file_info(&f) {
            Ok(i) => Some(i),
            Err(_) => None
        }
    }).collect()
}

//Load tags from file
fn load_file_info(path: &str) -> Result<AudioFileInfo, Box<dyn Error>> {
    //Load FLAC
    if path.to_ascii_lowercase().ends_with(".flac") {
        return load_flac_info(path);
    }

    //MP3
    if path.to_ascii_lowercase().ends_with(".mp3") {
        return load_id3_info(path, &id3::Tag::read_from_path(path)?, AudioFormat::MP3);
    }

    //AIFF
    load_id3_info(path, &id3::Tag::read_from_aiff(path)?, AudioFormat::AIFF)
}

fn load_flac_info(path: &str) -> Result<AudioFileInfo, Box<dyn Error>> {
    //Load header
    let mut file = File::open(path)?;
    let mut header: [u8; 4] = [0; 4];
    file.read_exact(&mut header)?;
    //Check for FLAC with ID3
    if &header[0..3] == b"ID3" {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "FLAC ID3 not supported!").into());
    }
    //Check if FLAC
    if &header != b"fLaC" {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Not a valid FLAC!").into());
    }
    file.seek(SeekFrom::Start(0))?;
    //Load tag
    let tag = metaflac::Tag::read_from(&mut file)?;
    let vorbis = tag.vorbis_comments().ok_or("Missing Vorbis data!")?;
    //Parse artists
    let artists = match vorbis.artist().ok_or("Missing artists!")?.len() {
        0 => None.ok_or("Missing artists!")?,
        //Single artist tag, manually parse
        1 => parse_artist_tag(vorbis.artist().unwrap().first().unwrap()),
        //Multiple, keep as is
        _ => vorbis.artist().unwrap().to_owned()
    };

    Ok(AudioFileInfo {
        path: path.to_owned(),
        title: vorbis.title().ok_or("Missing title!")?.first().ok_or("Missing title!")?.to_owned(),
        artists,
        format: AudioFormat::FLAC
    })
}

//Load tags from ID3
fn load_id3_info(path: &str, tag: &id3::Tag, format: AudioFormat) -> Result<AudioFileInfo, Box<dyn Error>> {
    Ok(AudioFileInfo {
        path: path.to_owned(),
        title: tag.title().ok_or("Missing title tag!")?.to_owned(),
        artists: parse_artist_tag(tag.artist().ok_or("Missing artist tag!")?),
        format
    })
}

//Wrapper to update tags
pub fn update_tags(track: &Track, info: &AudioFileInfo, config: &TaggerConfig) -> Result<(), Box<dyn Error>> {
    if info.format == AudioFormat::FLAC {
        return update_flac_tags(&info.path, track, config);
    }

    //ID3 Version
    let id3_version = match config.id3v24 {
        true => id3::Version::Id3v24,
        false => id3::Version::Id3v23
    };

    //MP3
    if info.format == AudioFormat::MP3 {
        let mut tag = id3::Tag::read_from_path(&info.path)?;
        update_id3_tags(&mut tag, track, config)?;
        tag.write_to_path(&info.path, id3_version)?;
        return Ok(());
    }

    //AIFF
    if info.format == AudioFormat::AIFF {
        let mut tag = id3::Tag::read_from_aiff(&info.path)?;
        update_id3_tags(&mut tag, track, config)?;
        tag.write_to_aiff(&info.path, id3_version)?;
        return Ok(());
    }

    Ok(())
}

fn update_flac_tags(path: &str, track: &Track, config: &TaggerConfig) -> Result<(), Box<dyn Error>> {
    //Load
    let mut tag = metaflac::Tag::read_from_path(path)?;
    let vorbis = tag.vorbis_comments_mut();

    if config.title && config.overwrite {
        vorbis.set_title(vec![&track.full_title()]);
    }
    if config.album && track.album.is_some() && (config.overwrite || vorbis.album().is_none()) {
        vorbis.set_album(vec![track.album.as_ref().unwrap()]);
    }
    if config.artists && config.overwrite {
        vorbis.set_artist(track.artists.clone());
    }
    if config.label && (config.overwrite || vorbis.get("LABEL").is_none()) {
        vorbis.set("LABEL", vec![&track.label]);
    }
    if config.bpm && track.bpm.is_some() && (config.overwrite || vorbis.get("BPM").is_none()) {
        vorbis.set("BPM", vec![&track.bpm.unwrap().to_string()]);
    }
    if config.genre && (config.overwrite || vorbis.genre().is_none()) {
        vorbis.set_genre(vec![&track.genre]);
    }
    if config.release_date && (config.overwrite || vorbis.get("DATE").is_none()) {
        vorbis.set("DATE", vec![&track.release_date]);
    }
    if config.key && track.key.is_some() && (config.overwrite || vorbis.get("INITIALKEY").is_none()) {
        vorbis.set("INITIALKEY", vec![track.key.as_ref().unwrap().replace("maj", "").replace("min", "m")]);
    }

    //Art
    if config.art && track.art.is_some() && (config.overwrite || tag.pictures().count() == 0) {
        match download_art(track.art.as_ref().unwrap()) {
            Ok(data) => {
                tag.remove_picture_type(FLACPictureType::CoverFront);
                tag.add_picture("image/jpeg", FLACPictureType::CoverFront, data);
            },
            Err(_) => eprintln!("Error downloading album art, ignoring!")
        }
    }

    //Save
    tag.save()?;
    Ok(())
}

fn update_id3_tags(tag: &mut id3::Tag, track: &Track, config: &TaggerConfig) -> Result<(), Box<dyn Error>> {
    if config.title && config.overwrite {
        tag.set_title(&track.full_title());
    }
    if config.album && track.album.is_some() && (config.overwrite || tag.album().is_none()) {
        tag.set_album(track.album.as_ref().unwrap());
    }
    if config.artists && config.overwrite {
        tag.set_artist(track.artists.join(&config.separator));
    }
    if config.label && (config.overwrite || tag.get("TPUB").is_none()) {
        tag.set_text("TPUB", &track.label);
    }

    //Parse date
    if config.release_date {
        if let Ok(date) = NaiveDate::parse_from_str(&track.release_date, "%Y-%m-%d") {
            //Date ID3 v2.4
            if config.id3v24 && (config.overwrite || tag.date_recorded().is_none()) {
                //Remove ID3v2.3
                tag.remove("TDAT");
                tag.remove("TYER");

                tag.set_date_recorded(id3::Timestamp {
                    year: date.year(),
                    month: Some(date.month() as u8),
                    day: Some(date.day() as u8),
                    hour: None,
                    minute: None,
                    second: None
                });
            }
            //Date ID3 v2.3
            if !config.id3v24 {
                if config.overwrite || tag.get("TDAT").is_none() {
                    tag.remove_date_recorded();
                    tag.set_text("TDAT", &format!("{:02}{:02}", date.day(), date.month()))
                }

                //Year
                if config.overwrite || tag.year().is_none() {
                    tag.remove_date_recorded();
                    tag.set_year(date.year());
                }
            }
        }
    }

    if config.genre && (config.overwrite || tag.genre().is_none()) {
        tag.set_genre(&track.genre);
    }
    if config.key && track.key.is_some() && (config.overwrite || tag.get("TKEY").is_none()) {
        tag.set_text("TKEY", &track.key.as_ref().unwrap().replace("maj", "").replace("min", "m"));
    }
    if config.bpm && track.bpm.is_some() && (config.overwrite || tag.get("TBPM").is_none()) {
        tag.set_text("TBPM", &track.bpm.unwrap().to_string());
    }

    //Art
    if config.art && track.art.is_some() && (config.overwrite || tag.pictures().count() == 0) {
        match download_art(track.art.as_ref().unwrap()) {
            Ok(data) => {
                tag.remove_picture_by_type(ID3PictureType::CoverFront);
                tag.add_picture(id3::frame::Picture {
                    mime_type: String::from("image/jpeg"),
                    picture_type: ID3PictureType::CoverFront,
                    description: String::from("Cover"),
                    data
                });
            },
            Err(_) => eprintln!("Error downloading album art, ignoring!")
        }
    }

    Ok(())
}

pub fn download_art(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let res = reqwest::blocking::get(url)?;
    Ok(res.bytes()?.to_vec())
}

#[derive(Clone, Debug)]
pub struct AudioFileInfo {
    pub path: String,
    pub title: String,
    pub artists: Vec<String>,
    pub format: AudioFormat
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioFormat {
    MP3,
    FLAC,
    AIFF
}

//Try to split artist string with common separators
fn parse_artist_tag(src: &str) -> Vec<String> {
    if src.contains(';') {
        return src.split(';').collect::<Vec<&str>>().into_iter().map(|v| v.to_owned()).collect();
    }
    if src.contains(',') {
        return src.split(',').collect::<Vec<&str>>().into_iter().map(|v| v.to_owned()).collect();
    }
    if src.contains('/') {
        return src.split('/').collect::<Vec<&str>>().into_iter().map(|v| v.to_owned()).collect();
    }
    vec![src.to_owned()]
}