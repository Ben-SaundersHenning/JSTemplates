// local settings

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

#[tauri::command]
pub fn get_settings(app_handle: tauri::AppHandle) -> Option<Settings> {

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


            //open file
            let config = File::open(&settings_file_path).unwrap();
            let settings: Settings = serde_json::from_reader(config).unwrap();
            return Some(settings);

        },

        Err(e) => { error!(target: "app", "{}", e); }

    }

    None

}

fn create_settings_file(path: &PathBuf) {

    // create the parent directories
    if let Err(e) = create_dir_all(path.parent().unwrap()) {
        error!(target: "app", "{}", e);
    }

    // create the json file
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);
    let settings = Settings { save_dir: "".to_owned() };
    serde_json::to_writer(&mut writer, &settings).unwrap();
    writer.flush().unwrap();

    info!(target: "app", "Created a settings file with default options.");

}
