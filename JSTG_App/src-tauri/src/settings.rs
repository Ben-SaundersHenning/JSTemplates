use std::fs;
use serde::{Serialize, Deserialize};
use log::info;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Setting {
    save_dir: String
}

pub async fn get_save_dir(handle: tauri::AppHandle) -> String {
    let settings = handle.path_resolver().resolve_resource("../settings.json").expect("failed to load settings");
    info!(target: "app", "Reading the base save directory from the settings file: {0}", settings.to_str().unwrap());
    let config = fs::File::open(&settings).unwrap();
    let settings: Setting = serde_json::from_reader(config).unwrap();
    settings.save_dir
}
