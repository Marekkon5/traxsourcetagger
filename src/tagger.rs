extern crate regex;

use regex::Regex;
use threadpool::ThreadPool;
use std::error::Error;
use std::sync::mpsc::{channel, Receiver};
use strsim::normalized_levenshtein;

use crate::traxsource::{Traxsource, Track};
use crate::utils::AudioFileInfo;
use crate::utils;

#[derive(Debug, Clone)]
pub struct TaggerConfig {
    //Tags
    pub title: bool,
    pub artists: bool,
    pub label: bool,
    pub key: bool,
    pub bpm: bool,
    pub genre: bool,
    pub release_date: bool,
    pub album: bool,
    pub art: bool,

    //Other
    pub fuzziness: u8,
    pub overwrite: bool,
    pub id3v24: bool,
    pub separator: String,
}

//Tag list of tracks, returns receive channel
pub fn tag_tracks(traxsource: &Traxsource, tracks: &Vec<AudioFileInfo>, config: &TaggerConfig) -> Result<Receiver<(bool, utils::AudioFileInfo)>, Box<dyn Error>> {
    //Initialize pool
    let thread_count = 8;
    let track_count = tracks.len() as usize;
    let pool = ThreadPool::new(thread_count);

    let (tx, rx) = channel();
    for i in 0..track_count {
        let tx = tx.clone();
        let info = tracks[i].clone();
        let c = config.clone();
        let t = traxsource.clone();
        pool.execute(move || {
            //Tag track
            match tag_track(&t, &info, &c) {
                Ok(_) => tx.send((true, info)).ok(),
                Err(_) => tx.send((false, info)).ok()
            };
        });
    }

    Ok(rx)
}

//Match and tag single track
pub fn tag_track(traxsource: &Traxsource, info: &AudioFileInfo, config: &TaggerConfig) -> Result<(), Box<dyn Error>> {
    let track = match_track(&traxsource, &info, &config)?.ok_or("No match found!")?;
    utils::update_tags(&track, &info, &config)?;
    Ok(())
}

pub fn match_track(traxsource: &Traxsource, info: &AudioFileInfo, config: &TaggerConfig) -> Result<Option<Track>, Box<dyn Error>> {
    //Search
    let results = traxsource.search_tracks(&format!("{} {}", 
        clean_title(&info.title, false), 
        &info.artists.first().unwrap()
    ))?;

    if results.len() == 0 {
        return Ok(None);
    }
    
    //Calculate fuzziness
    let mut fuzz: Vec<(f64, Track)> = vec![];
    for track in results {
        let fuzz_title = normalized_levenshtein(
            &clean_title(&track.full_title(), true), 
            &clean_title(&info.title, true)
        ) * 100_f64;
        if fuzz_title >= config.fuzziness as f64 {
            if match_artist(&info.artists, &track.artists) {
                fuzz.push((fuzz_title, track));
            }
        }
    }
    //No results
    if fuzz.len() == 0 {
        return Ok(None)
    }
    //Sort
    fuzz.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut track = fuzz.first().unwrap().1.clone();

    //Fetch extra meta if required
    if config.album || config.art {
        traxsource.extend_track(&mut track).ok();
    }

    Ok(Some(track))
}

fn clean_title(title: &str, matching: bool) -> String {
    let lowercase = title.to_lowercase();
    //Remove original mix
    let re = Regex::new(r"(\(*)original( (mix|version|edit))*(\)*)$").unwrap();
    let m0 = re.replace(&lowercase, "");
    //Remove edit
    let out = m0.to_string().replace("edit", "");

    if matching {
        //Remove space, some problematic characters
        return out.replace(" ", "").replace(";", "").replace("&", "")
    }
    out
}

//Match atleast 1 artist
fn match_artist(a: &Vec<String>, b: &Vec<String>) -> bool {
    let bb: Vec<String> = b.into_iter().map(|e| e.to_ascii_lowercase().replace(" ", "")).collect();
    for aa in a {
        if bb.contains(&aa.to_ascii_lowercase().replace(" ", "")) {
            return true;
        }
    }

    false
}
