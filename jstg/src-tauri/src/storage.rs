use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::fs::File;
use std::{fs::create_dir_all, path::Path};
use log::{info, error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    save_dir: String
}

impl Settings {

    // Creates an empty Settings
    // struct.
    fn new() -> Self {
        Self {
            save_dir: "".into(),
        }
    }

}

#[tauri::command]
pub fn get_settings(app_handle: tauri::AppHandle) -> Option<Settings> {

    let app_name = &app_handle.package_info().name;

    let mut settings_file_path = Path::new(&tauri::api::path::config_dir().unwrap())
        .join(app_name);

    // append the file to the directory path
    settings_file_path.push("settings.json");

    // note that try_exists OK branch returns a true or false
    // for existance.
    match settings_file_path.try_exists() {

        Ok(exists) => {

            if !exists {
                create_settings_file(&settings_file_path);
            }

            // open file, read to Settings struct
            let config = File::open(&settings_file_path).unwrap();
            let settings: Settings = serde_json::from_reader(config).unwrap();

            return Some(settings);

        },

        Err(e) => { error!(target: "app", "{}", e); }

    }

    None

}

// Writes a Settings object to JSON file.
// Note that the file is currently overwritten every time, but no
// unchanged data is lost.
// new_settings is a json string.
#[tauri::command]
pub fn update_settings(app_handle: tauri::AppHandle, new_settings: String) {

    let app_name = &app_handle.package_info().name;

    let mut settings_file_path = Path::new(&tauri::api::path::config_dir().unwrap())
        .join(app_name);

    // append the file to the directory path
    settings_file_path.push("settings.json");

    match settings_file_path.try_exists() {

        Ok(exists) => {

            if !exists {
                create_settings_file(&settings_file_path);
            }

            // open file
            let file = File::create(settings_file_path).unwrap();

            // convert settings to struct
            let settings: Settings = serde_json::from_str(&new_settings).unwrap();

            // write new settings to file
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &settings).unwrap();
            writer.flush().unwrap();

            info!(target: "app", "Wrote new settings to settings file.");

        },

        Err(e) => { error!(target: "app", "{}", e); }

    }

}

// Creates an empty settings file.
fn create_settings_file(path: &PathBuf) {

    // create the parent directories
    if let Err(e) = create_dir_all(path.parent().unwrap()) {
        error!(target: "app", "{}", e);
    }

    // create the json file
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);
    let settings = Settings::new();
    serde_json::to_writer(&mut writer, &settings).unwrap();
    writer.flush().unwrap();

    info!(target: "app", "Created a settings file with default options.");

}
