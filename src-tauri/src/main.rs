// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ path::{ Path, PathBuf }, fs::{ self, File }, error::Error, clone };
use serde::{ Deserialize, Serialize };
use figment::{ Figment, providers::{ Format, Toml, Json, Env } };
use once_cell::sync::OnceCell;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn getBasePath() -> String {
    let config = get_config().unwrap();
    config.base_path
}

#[tauri::command]
fn getFolders() -> Vec<String> {
    let config: Config = get_config().unwrap();
    let folders: Vec<&str> = config.folders
        .iter()
        .map(|folder: &Folders| folder.path.as_str())
        .collect();

    folders
        .iter()
        .map(|folder| folder.to_string())
        .collect()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Folders {
    name: String,
    path: String,
    extensions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    folders: Vec<Folders>,
    base_path: String,
}

impl Config {
    fn default(base_path_dir: String) -> Config {
        let delimeter = if cfg!(windows) { "\\" } else { "/" };
        Config {
            folders: vec![
                Folders {
                    name: "Application".to_string(),
                    path: format!("{}{}Application", base_path_dir, delimeter),
                    extensions: vec!["exe".to_string(), "msi".to_string(), "dmg".to_string()],
                },
                Folders {
                    name: "Documents".to_string(),
                    path: format!("{}{}Documents", base_path_dir, delimeter),
                    extensions: vec!["pdf".to_string(), "doc".to_string(), "docx".to_string()],
                },
                Folders {
                    name: "Media".to_string(),
                    path: format!("{}{}Media", base_path_dir, delimeter),
                    extensions: vec!["mp3".to_string(), "mp4".to_string(), "mkv".to_string()],
                },
                Folders {
                    name: "Images".to_string(),
                    path: format!("{}{}Images", base_path_dir, delimeter),
                    extensions: vec!["jpg".to_string(), "png".to_string(), "gif".to_string()],
                },
                Folders {
                    name: "Archive".to_string(),
                    path: format!("{}{}Archive", base_path_dir, delimeter),
                    extensions: vec!["zip".to_string(), "rar".to_string(), "gz".to_string()],
                },
                Folders {
                    name: "Other".to_string(),
                    path: format!("{}{}Other", base_path_dir, delimeter),
                    extensions: vec!["".to_string()],
                }
            ],
            base_path: base_path_dir,
        }
    }
}

fn main() {
    let mut loaded_config: Option<Config> = None;

    match get_config() {
        Ok(config) => {
            loaded_config = Some(config);
        }
        Err(e) => println!("Error running config manager: {:?}", e),
    }

    if let Some(config) = &loaded_config {
        let list_of_files = list_all_files_in_downloads(config.clone());
        let created_folders: Result<Vec<(String, PathBuf)>, std::io::Error> = create_folders(
            config.clone()
        );
        if created_folders.is_err() {
            println!("Error creating folders: {:?}", created_folders.err());
        }
        move_files_to_folders(list_of_files, config.clone());

        //TODO: Test out updateConfig
        // #[warn(unused_mut)]
        // let mut test_path = "C://test";
        // let mut new_config = config.clone();
        // new_config.base_path = test_path.to_string();
        // update_config(&mut new_config);
    } else {
        println!("Error: Config not loaded.");
    }

    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, getBasePath, getFolders])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn list_all_files_in_downloads(config: Config) -> Vec<String> {
    let mut files = Vec::new();
    let path_from_config: String = config.base_path.clone();
    let path = Path::new(path_from_config.as_str());
    println!("Path: {}", path.display());

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

fn create_folders(config: Config) -> std::io::Result<Vec<(String, PathBuf)>> {
    println!("Creating folders");
    let folders = config.folders.iter().map(|folder| folder.name.as_str());
    let mut created_folders: Vec<(String, PathBuf)> = vec![];
    println!("{:?}", folders);
    for folder in folders {
        let mut folder_path = PathBuf::new();
        folder_path.push(config.base_path.clone());
        folder_path.push(folder);

        // let folder_path = format!("{}\\{}", CONFIG.get().unwrap().base_path, folder);
        let path: &Path = Path::new(&folder_path);
        println!("Checking if folder exists: {}", path.to_str().unwrap());
        if !path.exists() {
            println!("CREATING FOLDER {}", folder);
            fs::create_dir(&folder_path)?;
            created_folders.push((folder.to_string(), path.to_path_buf()));
        } else {
            created_folders.push((folder.to_string(), path.to_path_buf()));
        }
    }

    Ok(created_folders)
}

fn update_config(new_config: &mut Config) {
    write_to_config(new_config.clone());
}

fn move_files_to_folders(files: Vec<String>, config: Config) {
    for file in files.iter() {
        if Path::new(file).extension().is_none() {
            continue;
        }
        let file_extension = Path::new(file).extension().expect("Unable to get file extension");

        let folder_to_move = config.folders
            .iter()
            .find(|folder| {
                folder.extensions.iter().any(|extension| extension.as_str() == file_extension)
            });

        if folder_to_move.is_none() {
            continue;
        }

        let folder_path = folder_to_move.unwrap().path.clone();
        let file_name = Path::new(file).file_name().expect("Unable to get file name");
        let file_name = file_name.to_str().expect("Unable to convert file name to string");

        let mut file_path = PathBuf::new();
        file_path.push(folder_path);
        file_path.push(file_name);

        println!("Moving file: {} to {}", file, file_path.display());

        fs::rename(file, file_path).expect("Unable to move file to archive folder");
    }
}

// TODO: Move this into a separate module
fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new("config.json");
    let home_dir: Option<PathBuf> = std::env::home_dir();
    let current_os: String = get_os();
    let base_path_dir = match current_os.as_str() {
        "windows" => {
            let path = home_dir.unwrap();
            let home_dir_path = format!("{}\\Downloads", path.display());
            home_dir_path
        }
        "linux" => {
            let path = home_dir.unwrap();
            let home_dir_path = format!("{}/Downloads", path.display());
            home_dir_path
        }
        _ => {
            let path = home_dir.unwrap();
            let home_dir_path = format!("{}/Downloads", path.display());
            home_dir_path
        }
    };

    if !config_path.exists() {
        let base_config: Config = Config::default(base_path_dir);
        let config_json = serde_json
            ::to_string(&base_config)
            .expect("Unable to serialize config to JSON");
        fs::write("config.json", config_json).expect("Unable to write config to file");
    }

    let config_file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(config_file)?;
    return Ok(config);
}

fn write_to_config(new_config: Config) {
    // Get the current config and then update the json file
    let config_path = Path::new("config.json");
    let config_file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to parse config file");

    let config_json = serde_json
        ::to_string(&new_config)
        .expect("Unable to serialize config to JSON");

    fs::write("config.json", config_json).expect("Unable to write config to file");
}