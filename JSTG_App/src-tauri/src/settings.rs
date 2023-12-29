use std::fs;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Setting {
    save_dir: String
}

pub async fn get_save_dir() -> String {

    let mut path = std::env::current_dir().unwrap();
    path.set_file_name("src-tauri/settings.json");
    let config = fs::read_to_string(path.to_str().unwrap()).unwrap();
    let settings: Setting = serde_json::from_str(config.as_ref()).unwrap();
    settings.save_dir

}
