use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;
use std::{fs::create_dir_all, path::Path};

extern crate dirs;

const APP_NAME: &str = "Jstg";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    path: String, // Path to the settings.json file
    settings: HashMap<String, String>, // Key-Value settings
}

impl Settings {

    // Opens the existing settings file,
    // or creates a new default one if one doesn't exist.
    fn open() -> Self {

        let settings_file_path: PathBuf = get_settings_file_path();

        match settings_file_path.try_exists() {

            Ok(truthy) => {

                if truthy {

                    // file exists, try to read it
                    let mut contents: String = fs::read_to_string(settings_file_path).unwrap_or("".to_string());
                    // serde_json::from_str(s)

                } else {

                    // file does not exist, create default hash
                    // and save it to new file

                }

            },
            Err(e) => {
                error!(target: "app", "{}", e);
            }

        }

        // Self {
        //     path: settings_file_path.to_str().unwrap().into(),
        // }

        todo!()

    }

    // Gets the associated value
    // for a given key
    pub fn get(&self, key: &str) -> &str {

        todo!()

    }

    // Sets the value for the given key
    pub fn set(&self, key: &str, value: &str) -> bool {

        todo!()

    }

    fn save(&self) {

        todo!()

    }

}

// #[tauri::command]
// pub fn get_settings(_app_handle: tauri::AppHandle) -> Option<Settings> {
//
//     // let app_name = &app_handle.package_info().name;
//
//     let settings_file_path: PathBuf = get_settings_file_path();
//
//     // note that try_exists OK branch returns a true or false
//     // for existance.
//     match settings_file_path.try_exists() {
//         Ok(exists) => {
//             if !exists {
//                 create_default_settings_file(&settings_file_path);
//             }
//
//             // open file, read to Settings struct
//             let config = File::open(&settings_file_path).unwrap();
//             let settings: Settings = serde_json::from_reader(config).unwrap();
//
//             return Some(settings);
//         }
//
//         Err(e) => {
//             error!(target: "app", "{}", e);
//         }
//     }
//
//     None
// }

// Writes a Settings object to JSON file.
// Note that the file is currently overwritten every time, but no
// unchanged data is lost.
// new_settings is a json string.
// #[tauri::command]
// pub fn update_settings(_app_handle: tauri::AppHandle, new_settings: String) {
//
//     // let app_name = &app_handle.package_info().name;
//
//     let settings_file_path: PathBuf = get_settings_file_path();
//
//     match settings_file_path.try_exists() {
//         Ok(exists) => {
//             if !exists {
//                 create_default_settings_file(&settings_file_path);
//             }
//
//             // open file
//             let file = File::create(settings_file_path).unwrap();
//
//             // convert settings to struct
//             let settings: Settings = serde_json::from_str(&new_settings).unwrap();
//
//             // write new settings to file
//             let mut writer = BufWriter::new(file);
//             serde_json::to_writer(&mut writer, &settings).unwrap();
//             writer.flush().unwrap();
//
//             info!(target: "app", "Wrote new settings to settings file.");
//         }
//
//         Err(e) => {
//             error!(target: "app", "{}", e);
//         }
//     }
// }

// Creates an empty settings file.
// fn create_default_settings_file(path: &PathBuf) {
//
//     // create the parent directories
//     if let Err(e) = create_dir_all(path.parent().unwrap()) {
//         error!(target: "app", "{}", e);
//     }
//
//     // create the json file
//     let file = File::create(path).unwrap();
//     let mut writer = BufWriter::new(file);
//     let settings = Settings::new();
//     serde_json::to_writer(&mut writer, &settings).unwrap();
//     writer.flush().unwrap();
//
//     info!(target: "app", "Created a settings file with default options.");
//
// }

fn get_settings_file_path() -> PathBuf {

    // Config Dir
    // Linux: $HOME/.config
    // Windows: RoamingAddData
    let mut settings_file_path = Path::new(&dirs::config_dir().unwrap()).join(APP_NAME);

    // append the file to the directory path
    settings_file_path.push("settings.json");

    settings_file_path

}
