// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ path::{ Path, PathBuf }, fs::{ self, File }, error::Error };
use serde::{ Deserialize, Serialize };
use figment::{ Figment, providers::{ Format, Toml, Json, Env } };
use once_cell::sync::OnceCell;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Serialize, Deserialize)]
struct Folders {
    name: String,
    path: String,
    extensions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    folders: Vec<Folders>,
    base_path: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

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
    fn update_config(&mut self, new_base_path_dir: String) {
        let delimeter = if cfg!(windows) { "\\" } else { "/" };

        for folder in self.folders.iter_mut() {
            let old_path = folder.path.clone();

            // Replace the old base path with the new one in the folder paths
            let new_path = old_path.replace(&self.base_path, &new_base_path_dir);

            // Replace the delimiter with the correct one for the current platform
            let new_path = new_path.replace("/", delimeter).replace("\\", delimeter);

            folder.path = new_path;
        }

        // Update the base path
        self.base_path = new_base_path_dir;
    }
}

fn main() {
    match get_config() {
        Ok(config) => {
            CONFIG.set(config).unwrap();
        }
        Err(e) => println!("Error running config manager: {:?}", e),
    }

    let list_of_files = list_all_files_in_downloads();
    let created_folders: Result<Vec<(String, PathBuf)>, std::io::Error> = create_folders();
    if created_folders.is_err() {
        println!("Error creating folders: {:?}", created_folders.err());
    }
    move_files_to_folders(list_of_files);

    //TODO: Test out updateConfig
    let new_config: &Config = CONFIG.get().unwrap();
    println!("{:?}", new_config);
    new_config.base_path = "C:\\TEST".to_string();
    println!("{:?}", new_config);

    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn list_all_files_in_downloads() -> Vec<String> {
    let mut files = Vec::new();
    let path_from_config: String = CONFIG.get().unwrap().base_path.clone();
    let path = Path::new(path_from_config.as_str());

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
    let folders = CONFIG.get()
        .unwrap()
        .folders.iter()
        .map(|folder| folder.name.as_str());
    let mut created_folders: Vec<(String, PathBuf)> = vec![];

    for folder in folders {
        let folder_path = format!("{}\\{}", CONFIG.get().unwrap().base_path, folder);
        let path: &Path = Path::new(&folder_path);
        if !path.exists() {
            fs::create_dir(&folder_path)?;
            created_folders.push((folder.to_string(), path.to_path_buf()));
        } else {
            created_folders.push((folder.to_string(), path.to_path_buf()));
        }
    }

    Ok(created_folders)
}

fn move_files_to_folders(files: Vec<String>) {
    for file in files.iter() {
        let file_extension = Path::new(file).extension().expect("Unable to get file extension");

        let folder_to_move = CONFIG.get()
            .unwrap()
            .folders.iter()
            .find(|folder| {
                folder.extensions.iter().any(|extension| extension.as_str() == file_extension)
            });

        if folder_to_move.is_none() {
            continue;
        }

        let folder_path = folder_to_move.unwrap().path.clone();
        let file_name = Path::new(file).file_name().expect("Unable to get file name");
        let file_name = file_name.to_str().expect("Unable to convert file name to string");
        let file_path = format!("{}\\{}", folder_path, file_name);
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

// function that accepts a new config and updates the current one in config.json
fn update_config(config: Config) -> Result<(), Box<dyn Error>> {
    let config_json = serde_json::to_string(&config).expect("Unable to serialize config to JSON");
    fs::write("config.json", config_json).expect("Unable to write config to file");
    Ok(())
}