extern crate web_view;
extern crate tinyfiledialogs;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::path::Path;
use std::time::{SystemTime, Duration};
use tungstenite::server::accept;
use tungstenite::{Message, WebSocket};
use serde_json::{Value, json};
use webbrowser;

use crate::tagger::TaggerConfig;
use crate::traxsource::Traxsource;
use crate::{utils, tagger};

pub fn start_ui() {
    let content = include_str!("../client/dist/dist.html");

    let webview = web_view::builder()
        .invoke_handler(|_, __| Ok(()))
        .content(web_view::Content::Html(content))
        .user_data(())
        .title("Traxsource Tagger")
        .size(400, 650)
        .resizable(false)
        .debug(true)
        .build()
        .unwrap();

    //Start socket server
    thread::spawn(|| {
        start_socket();
    });

    webview.run().unwrap();
}

//WebSockets server for communication to UI
pub fn start_socket() {
    let server = TcpListener::bind("127.0.0.1:36910").unwrap();
    for stream in server.incoming() {
        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_text() {
                    match handle_messange(msg.to_text().unwrap(), &mut websocket) {
                        Ok(_) => {},
                        Err(v) => {
                            //Send error to UI
                            eprintln!("{}", v);
                        }
                    }
                }
            }
        });
    }
}

//Websocket messange handler
fn handle_messange(text: &str, websocket: &mut WebSocket<TcpStream>) -> Result<(), String> {
    //Parse JSON
    let json: Value = serde_json::from_str(text).unwrap();
    match json["action"].as_str().unwrap() {
        //Update path
        "browse" => {
            let path = tinyfiledialogs::select_folder_dialog("Select folder", ".");
            if path.is_some() {
                websocket.write_message(Message::from(json!({
                    "action": "path",
                    "path": path
                }).to_string())).ok();
            }
        },
        //Validate path
        "validate" => {
            let path = json["path"].as_str().unwrap();
            if !Path::new(path).is_dir() {
                websocket.write_message(Message::from(json!({
                    "action": "validate",
                    "status": false
                }).to_string())).ok();
                return Ok(());
            }
            //Success
            websocket.write_message(Message::from(json!({
                "action": "validate",
                "status": true
            }).to_string())).ok();
        },
        //Open external url in browser
        "url" => {
            webbrowser::open(json["url"].as_str().unwrap()).ok();
        },
        //Exit from UI
        "exit" => {
            std::process::exit(0);
        },
        "start" => {
            println!("Starting...\n");
            
            let config_data = json["config"].as_object().unwrap();
            //Check path
            let path = json["path"].as_str().unwrap();
            if !Path::new(path).is_dir() {
                return Err(String::from("Invalid path!"));
            }
            //Load config
            let config = TaggerConfig {
                title: config_data["title"].as_bool().unwrap(),
                artists: config_data["artists"].as_bool().unwrap(),
                genre: config_data["genre"].as_bool().unwrap(),
                album: config_data["album"].as_bool().unwrap(),
                release_date: config_data["date"].as_bool().unwrap(),
                label: config_data["label"].as_bool().unwrap(),
                bpm: config_data["bpm"].as_bool().unwrap(),
                key: config_data["key"].as_bool().unwrap(),
                separator: String::from(config_data["separator"].as_str().unwrap()),
                fuzziness: config_data["fuzziness"].as_str().unwrap_or("80").parse().unwrap_or(80) as u8,
                art: config_data["art"].as_bool().unwrap(),
                overwrite: config_data["overwrite"].as_bool().unwrap(),
                id3v24: config_data["id3v24"].as_bool().unwrap(),
            };

            //Load files
            let files = utils::load_files(path);

            //Initialize
            let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_secs();
            let mut ok = 0;
            let mut fail = 0;
            let total = files.len();

            //Start
            let traxsource = Traxsource::new();
            let rx = tagger::tag_tracks(&traxsource, &files, &config).unwrap();
            for (succesful, info) in rx {
                //On progress
                if succesful {
                    ok += 1;
                } else {
                    fail += 1;
                }
                let eta = calculate_eta(start_time, ok + fail, total as u64);

                //Update UI
                websocket.write_message(Message::from(json!({
                    "action": "progress",
                    "progress": {
                        "ok": ok,
                        "fail": fail,
                        "total": total,
                        "eta": eta
                    },
                    "last": {
                        "path": info.path,
                        "ok": succesful
                    }
                }).to_string())).ok();
            }

            //Done
            websocket.write_message(Message::from(json!({
                "action": "done",
                "took": SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_secs() - start_time,
                "ok": ok,
                "fail": fail,
                "total": total,
            }).to_string())).ok();
        }
        _ => return Err(String::from("Invalid action!"))
    }

    Ok(())
}

//Calculate remaining seconds
fn calculate_eta(start_time: u64, currently: u64, total: u64) -> u64 {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::from_millis(0)).as_secs();
    let tpt = (current_time - start_time) as f64 / currently as f64;
    ((total - currently) as f64 * tpt) as u64
}