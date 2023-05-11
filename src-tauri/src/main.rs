// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path, fs, process::Command};

// static mut PATH: &str = "";

// Set the PATH to be used by the app, check first which OS I'm using and depending on that go to the Downloads folder


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    list_all_files_in_downloads();
    let currentOs = get_os();
    println!("The operating system is: {}", currentOs);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn list_all_files_in_downloads() -> Vec<String> {
    let mut files = Vec::new();
    let path = Path::new("/Users/almin/Downloads");
    println!("{:?}", path);
    for entry in fs::read_dir(path).expect("Unable to read dir") {
        let entry = entry.expect("Unable to get entry");
        let path = entry.path();
        if path.is_dir() {
           continue;
        }
        let path_str = path.to_str().expect("Unable to convert path to string");
        files.push(path_str.to_string());
    }
    println!("{:?}", files);
    files
}

fn get_os() -> String {
    let os = std::env::consts::OS;
    os.to_string()
}


