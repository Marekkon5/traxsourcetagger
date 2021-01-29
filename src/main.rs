#![windows_subsystem = "windows"]

extern crate msgbox;

use std::process::Command;
use std::process;
use msgbox::IconType;

mod traxsource;
mod tagger;
mod utils;
mod ui;

fn main() {
    if cfg!(windows) {
        windows_edge_setup();
    }

    ui::start_ui();
}

//Bug with webview and edge where loopback is disabled, check for it
fn windows_edge_setup() {
    //Check status
    let output = Command::new("cmd")
        .arg("/c")
        .arg("CheckNetIsolation.exe LoopbackExempt -s")
        .output()
        .unwrap();
    let output_text = String::from_utf8(output.stdout).unwrap();
    //Already allowed
    if output_text.to_lowercase().contains("_cw5n1h2txyewy") {
        return;
    }

    //Show msgbox
    msgbox::create(
        "Traxsource Tagger", 
        "Traxsource Tagger depends on local WebSocket connection to join UI with backend. However it is currently blocked by Edge Sandboxing. After pressing OK it will be enabled (requires Administrator privileges).",
        IconType::Info).ok();

    //Enable
    let status = runas::Command::new("cmd")
        .arg("/c CheckNetIsolation.exe LoopbackExempt -a -n=Microsoft.Win32WebViewHost_cw5n1h2txyewy")
        .status()
        .unwrap();

    //Failed
    if !status.success() {
        msgbox::create("Traxsource Tagger", "An error occured!", IconType::Error).ok();
        process::exit(-1);
    }
}