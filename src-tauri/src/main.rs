// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ path::{ Path, PathBuf }, fs::{ self, File }, error::Error };
use serde::{ Deserialize, Serialize };
use figment::{ Figment, providers::{ Format, Toml, Json, Env } };
use once_cell::sync::OnceCell;


// Set the PATH to be used by the app, check first which OS I'm using and depending on that go to the Downloads folder



// TODO: Figure out what OS im on and where the Download folder is

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


impl Config {
    fn default() -> Config {
        Config {
            folders: vec![
                Folders {
                    name: "Application".to_string(),
                    path: "C:\\Users\\almin\\Downloads\\Application".to_string(),
                    extensions: vec!["exe".to_string(), "msi".to_string(), "dmg".to_string()],
                },
                Folders {
                    name: "Documents".to_string(),
                    path: "C:\\Users\\almin\\Downloads\\Documents".to_string(),
                    extensions: vec!["pdf".to_string(), "doc".to_string(), "docx".to_string()],
                },
                Folders {
                    name: "Media".to_string(),
                    path: "C:\\Users\\almin\\Downloads\\Media".to_string(),
                    extensions: vec!["mp3".to_string(), "mp4".to_string(), "mkv".to_string()],
                },
                Folders {
                    name: "Images".to_string(),
                    path: "C:\\Users\\almin\\Downloads\\Images".to_string(),
                    extensions: vec!["jpg".to_string(), "png".to_string(), "gif".to_string()],
                },
                Folders {
                    name: "Archive".to_string(),
                    path: "C:\\Users\\almin\\Downloads\\Archive".to_string(),
                    extensions: vec!["zip".to_string(), "rar".to_string(), "gz".to_string()],
                },
                Folders {
                    name: "Other".to_string(),
                    path: "C:\\Users\\almin\\Downloads\\Other".to_string(),
                    extensions: vec!["".to_string()],
                }],
            base_path: "C:\\Users\\almin\\Downloads".to_string(),
        }
    }
}

static CONFIG: OnceCell<Config> = OnceCell::new();


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

    // let currentOs = get_os();
    // println!("The operating system is: {}", currentOs);

    move_files_to_folders(list_of_files);

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
    let folders = CONFIG.get().unwrap().folders.iter().map(|folder| folder.name.as_str());
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

        let folder_to_move = CONFIG.get().unwrap().folders.iter().find(|folder| {
          folder
                .extensions
                .iter()
                .any(|extension| extension.as_str() == file_extension)
        });

        if folder_to_move.is_none() {
            continue;
        }

        let folder_path = folder_to_move.unwrap().path.clone();
        let file_name = Path::new(file).file_name().expect("Unable to get file name");
        let file_name = file_name.to_str().expect("Unable to convert file name to string");
        let file_path = format!(
            "{}\\{}",
            folder_path,
            file_name
        );
        fs::rename(file, file_path).expect("Unable to move file to archive folder");

}
}

fn get_home_dir() {
            match home::home_dir() {
    Some(path) => println!("{}", path.display()),
    None => println!("Impossible to get your home dir!"),
}

}

// TODO: Move this into a separate module 
fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new("config.json");
    if !config_path.exists() {
        let base_config = Config::default();
        let config_json = serde_json
            ::to_string(&base_config)
            .expect("Unable to serialize config to JSON");
        fs::write("config.json", config_json).expect("Unable to write config to file");
    }

    let config_file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(config_file)?;
    return Ok(config);
}

fn update_config() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("config.json");
    let config_file = File::open(config_path)?;
    let config: Config = serde_json::from_reader(config_file)?;
    let config_json = serde_json
        ::to_string(&config)
        .expect("Unable to serialize config to JSON");
    fs::write("config.json", config_json).expect("Unable to write config to file");
    Ok(())
}