// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::{Path, PathBuf}, fs, error::Error};



// TODO: move to a config
const PATH: &str = "C:\\Users\\almin\\Downloads";
// Set the PATH to be used by the app, check first which OS I'm using and depending on that go to the Downloads folder

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let list_of_files = list_all_files_in_downloads();
    let created_folders = create_folders();
    if created_folders.is_err() {
        println!("Error creating folders: {:?}", created_folders.err());
    }
    let currentOs = get_os();
    println!("The operating system is: {}", currentOs);

    // Move files to their respective folders
    move_files_to_folders(self::create_folders().unwrap().as_ref(), list_of_files);
    

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn list_all_files_in_downloads() -> Vec<String> {
    let mut files = Vec::new();
    let path = Path::new(PATH);
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

fn create_folders() -> std::io::Result<Vec<(String, PathBuf)>> {
    // TODO: Move the folders to a config file
    let folders = ["Application", "Documents", "Media", "Images", "Archive", "Other"];
    let mut created_folders: Vec<(String, PathBuf)> = vec![];

    for folder in folders.iter() {
        let folder_path = format!("{}\\{}", PATH, folder);
        let path: &Path = Path::new(&folder_path);
        if !path.exists() {
            fs::create_dir(&folder_path)?;
            created_folders.push((folder.to_string(), path.to_path_buf()));
        } else {
            created_folders.push((folder.to_string(), path.to_path_buf()));
        }
        println!("Created folder: {}", folder_path);
    }

    //TODO: save the changes here to the config
    Ok(created_folders)

}


fn move_files_to_folders(folders: &Vec<(String, PathBuf)>, files: Vec<String>) { 
    //TODO: Move the extensions to a specific config
    let extensions = [".exe", ".pdf", ".docx", ".mp3", ".mp4", ".png", ".jpg", ".zip", ".rar", ".7z", ".txt"];

    // check the extension of the files and move them to the respective folder
    for file in files.iter() {
        // Get files extension 
        let file_extension = Path::new(file).extension().expect("Unable to get file extension");
        if(file_extension == "zip" || file_extension == "rar" || file_extension == "gz") {
            let archive_folder = folders.iter().find(|(name, _)| name == "Archive").expect("Unable to find archive folder");
            let file_name = Path::new(file).file_name().expect("Unable to get file name");
            let file_name = file_name.to_str().expect("Unable to convert file name to string");
            let file_path = format!("{}\\{}", archive_folder.1.to_str().expect("Unable to convert archive folder path to string"), file_name);
            fs::rename(file, file_path).expect("Unable to move file to archive folder");
        }
    }
}


