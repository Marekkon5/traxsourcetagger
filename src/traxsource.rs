extern crate minify_html;

use reqwest::blocking::Client;
use reqwest::Method;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct Traxsource {
    client: Client
}

impl Traxsource {
    pub fn new() -> Traxsource {
        //Setup HTTP client
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 Safari/537.36")
            .build()
            .unwrap();

        Traxsource {
            client: client,
        }
    }

    pub fn search_tracks(&self, query: &str) -> Result<Vec<Track>, Box<dyn Error>> {
        //Fetch
        let mut data = self.client.request(Method::GET, "https://www.traxsource.com/search/tracks")
            .query(&[("term", query)])
            .send()?
            .text()?;
        
        //Minify and parse
        minify_html::in_place_str(&mut data, &minify_html::Cfg {minify_js: false}).unwrap();
        let document = Html::parse_document(&data);

        //Track list
        let list_selector = Selector::parse("div#searchTrackList").unwrap();
        let track_list = document.select(&list_selector).next().ok_or("No results!")?;
        //Select track
        let track_selector = Selector::parse("div.trk-row").unwrap();
        let mut tracks = vec![];
        for track_element in track_list.select(&track_selector) {
            //Get title
            let mut selector = Selector::parse("div.title").unwrap();
            let title_elem = track_element.select(&selector).next().unwrap();
            let title_vec = title_elem.text().collect::<Vec<_>>();
            let title = title_vec[0].to_owned();
            let version = match title_vec.len() {
                3 => {
                    //Remove space at end because of duration
                    let mut v = title_vec[1].to_owned();
                    v.pop();
                    Some(v)
                },
                _ => None
            };

            //Get URL
            selector = Selector::parse("a").unwrap();
            let title_link = title_elem.select(&selector).next().unwrap();
            let url = title_link.value().attr("href").unwrap();

            //Artists
            selector = Selector::parse("div.artists a").unwrap();
            let artists: Vec<String> = track_element.select(&selector).map(|e| {
                e.text().collect::<Vec<_>>().first().unwrap().to_owned().to_owned()
            }).collect();
            
            //Label
            selector = Selector::parse("div.label").unwrap();
            let label = track_element.select(&selector).next().unwrap().text().collect::<Vec<_>>().first().unwrap().to_owned();
            
            //Key, BPM
            selector = Selector::parse("div.key-bpm").unwrap();
            let key_bpm_values = track_element.select(&selector).next().unwrap().text().collect::<Vec<_>>();
            let mut key = None;
            let mut bpm = None;
            if key_bpm_values.len() == 2 {
                key = Some(key_bpm_values[0].to_owned());
                bpm = Some(key_bpm_values[1].to_owned().parse().unwrap());
            }
            
            //Genre
            selector = Selector::parse("div.genre").unwrap();
            let genre = track_element.select(&selector).next().unwrap().text().collect::<Vec<_>>().first().unwrap().to_owned();

            //Release date
            selector = Selector::parse("div.r-date").unwrap();
            let release_date = track_element.select(&selector).next().unwrap().text().collect::<Vec<_>>().first().unwrap().to_owned();

            tracks.push(Track {
                title: title,
                version: version,
                url: url.to_owned(),
                artists: artists,
                label: label.to_owned(),
                key: key,
                bpm: bpm,
                genre: genre.to_owned(),
                release_date: release_date.to_owned(),
                album: None,
                art: None
            });
        }

        Ok(tracks)
    }

    //Tracks in search don't have album name, album art
    pub fn extend_track(&self, track: &mut Track) -> Result<(), Box<dyn Error>> {
        //Fetch
        let mut data = self.client.request(Method::GET, &format!("https://www.traxsource.com{}", track.url))
            .send()?
            .text()?;
        
        //Minify and parse
        minify_html::in_place_str(&mut data, &minify_html::Cfg {minify_js: false}).unwrap();
        let document = Html::parse_document(&data);

        //Select album element
        let mut selector = Selector::parse("div.ttl-info.ellip a").unwrap();
        let album_element = document.select(&selector).next().unwrap();
        let album_text = album_element.text().collect::<Vec<_>>();
        track.album = Some(album_text.first().unwrap().to_owned().to_owned());

        //Select album art element
        selector = Selector::parse("div.tr-image img").unwrap();
        let img_element = document.select(&selector).next().unwrap();
        let art_url = img_element.value().attr("src").unwrap();
        track.art = Some(art_url.to_owned());

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Track {
    pub title: String,
    pub version: Option<String>,
    pub artists: Vec<String>,
    pub label: String,
    pub key: Option<String>,
    pub bpm: Option<i16>,
    pub genre: String,
    pub release_date: String,
    pub album: Option<String>,
    pub art: Option<String>,
    pub url: String
}

impl Track {
    //Get full track title with version
    pub fn full_title(&self) -> String {
        match &self.version {
            Some(v) => {
                if v.len() > 0 && v != " " {
                    format!("{} ({})", &self.title, v)
                } else {
                    self.title.to_owned()
                }
            }
            None => self.title.to_owned()
        }
    }
}