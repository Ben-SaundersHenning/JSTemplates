use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::{fs::create_dir_all, path::Path};

extern crate dirs;

const APP_NAME: &str = "Jstg";

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    path: String, // Path to the settings.json file
    settings: HashMap<String, String>, // Key-Value settings
}

impl Settings {

    // Opens the existing settings file,
    // or creates a new default one if one doesn't exist.
    pub fn open() -> Self {

        let settings_file_path: PathBuf = get_settings_file_path();

        match settings_file_path.try_exists() {

            Ok(exists) => {

                if exists {

                    let mut settings = Settings {
                        path: settings_file_path.clone().into_os_string().into_string().unwrap(),
                        settings: HashMap::new(),
                    };

                    // insert each setting into the hashmap
                    for line in fs::read_to_string(settings_file_path).unwrap().lines() {
                        let key_vals: Vec<&str> = line.split(":")
                                        .map(|x| x.trim())
                                        .collect();
                        settings.settings.insert(key_vals[0].to_string(), key_vals[1].to_string());
                    }

                    info!(target: "app", "Loaded the existing configuration file.");

                    return settings;

                // the file does not exist
                } else {

                    // create the parent directories
                    if let Err(e) = create_dir_all(settings_file_path.parent().unwrap()) {
                        error!(target: "app", "{}", e);
                    }

                    // create the file
                    let file = File::create(settings_file_path.clone()).unwrap();
                    let mut writer = BufWriter::new(file);

                    // default settings
                    let settings = Settings {
                        path: settings_file_path.into_os_string().into_string().unwrap(),
                        settings: HashMap::from([
                            ("document_save_path".to_string(), get_documents_path().into_os_string().into_string().unwrap())
                        ])
                    };

                    for (key, value) in settings.clone().settings.into_iter() {
                        write!(&mut writer, "{key}: {value}\n").unwrap();
                    }

                    writer.flush().unwrap();

                    info!(target: "app", "Created a configuration file with default options.");

                    return settings;

                }

            },
            Err(e) => {
                error!(target: "app", "{}", e);
            }

        }

        // Self {
        //     path: settings_file_path.to_str().unwrap().into(),
        // }

        // default settings
        let settings = Settings {
            path: get_settings_file_path().into_os_string().into_string().unwrap(),
            settings: HashMap::from([
                ("document_save_path".to_string(), get_documents_path().into_os_string().into_string().unwrap())
            ])
        };

        settings

    }

    // Gets the associated value
    // for a given key
    pub fn get(&self, key: &str) -> Option<&str> {

        match self.settings.get_key_value(key) {
            Some(val) => {
                return Some(val.1);
            },
            None => {}
        }

        None

    }

    // Returns every key-value pair in the config
    pub fn get_all(&self) -> Option<HashMap<String, String>> {

        return Some(self.settings.clone());

    }

    // Sets the value for the given key
    pub fn set(&mut self, key: &str, value: &str) -> bool {

        // overwrite or new key?
        todo!()

    }

    // Overwrites all configuration values
    pub fn set_all(&mut self, values: HashMap<String, String>) -> bool {

        self.settings = values;
        true

    }

    pub fn remove(&mut self, key: &str) -> bool {

        match self.settings.remove(key) {
            Some(_) => return true,
            None => return false
        }

    }

    // Writes the configuration values to file
    // Currently assumes that the file does exist
    fn save(&self) -> bool {

        let file = OpenOptions::new().write(true).truncate(true).open(get_settings_file_path()).unwrap();
        let mut writer = BufWriter::new(file);

        for (key, value) in self.settings.clone().into_iter() {
            write!(&mut writer, "{key}: {value}\n").unwrap();
        }

        writer.flush().unwrap();

        info!(target: "app", "Updated the configuration file.");

        true

    }

}

#[tauri::command]
pub fn get_config(_app_handle: tauri::AppHandle) -> HashMap<String, String> {

        let settings = Settings::open();

        settings.get_all().unwrap()

}

#[tauri::command]
pub fn update_config(conf: HashMap<String, String>) {

        let mut settings = Settings::open();

        settings.set_all(conf);

        settings.save();

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
    settings_file_path.push("configuration.txt");

    settings_file_path

}

fn get_documents_path() -> PathBuf {

    let file_path = Path::new(&dirs::document_dir().unwrap()).join(APP_NAME);

    file_path

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn open_configuration() {

        let settings = Settings::open();

    }

    #[test]
    #[ignore]
    fn get_value() {

        let settings = Settings::open();
        let result = settings.get("document_save_path").unwrap();
        assert_eq!(result, "test");

    }

    #[test]
    #[ignore]
    fn get_bad_value() {

        let settings = Settings::open();
        let result = settings.get("test123");
        assert!(result.is_none(), "true")

    }

    #[test]
    #[ignore]
    fn save_to_file() {

        let mut settings = Settings::open();
        settings.settings.insert("hello".to_string(), "world".to_string());
        settings.settings.insert("blahblah".to_string(), "yipee".to_string());

        let result = settings.save();

        assert_eq!(result, true);

    }

    #[test]
    #[ignore]
    fn remove_and_save_to_file() {

        let mut settings = Settings::open();
        settings.settings.insert("hello".to_string(), "world".to_string());
        settings.settings.insert("blahblah".to_string(), "yipee".to_string());

        settings.remove("test");

        let result = settings.save();

        assert_eq!(result, true);

    }

}
