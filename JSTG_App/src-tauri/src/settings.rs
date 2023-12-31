use std::env;
use std::path::{PathBuf, Path};
use std::{fs, io::Write};
use std::io::BufWriter; 
use serde::{Serialize, Deserialize};
use log::info;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Setting {
    save_dir: String
}

pub async  fn get_save_dir() -> String {

    let handle = super::get_app_handle();

    let mut settings_file_path = handle.path_resolver()
                            .app_config_dir()
                            .unwrap()
                            .into_os_string();

    settings_file_path.push("/settings.json");

    let path: PathBuf = PathBuf::from(settings_file_path);
    let parent = path.parent().unwrap();

    match path.try_exists() {
        Ok(val) => {
            if !val {
                create_log_file(&path, &parent);
            }
            info!(target: "app", "Reading the base save directory from the settings file: {0}", path.to_str().unwrap());
            let config = fs::File::open(&path).unwrap();
            let settings: Setting = serde_json::from_reader(config).unwrap();
            return settings.save_dir;
        },
        Err(_e) => {}
    };

    info!(target: "app", "Unable to read the base save directory from the settings file ({0})\nUsing system temp directory.", path.to_str().unwrap());
    return env::temp_dir().into_os_string().into_string().unwrap();

}

fn create_log_file(path: &PathBuf, parent: &Path) {
    info!(target: "app", "The log file ({0}) does not exist. Attempting to create now.", path.to_str().unwrap());
    if !parent.is_dir() {
        let _ = fs::create_dir_all(&parent);
    }
    let file = fs::File::create(&path).unwrap();
    let mut writer = BufWriter::new(file);
    let setting = Setting { save_dir: "/home/ben/Documents/JSTG_dev/".to_owned() };
    serde_json::to_writer(&mut writer, &setting).unwrap();
    writer.flush().unwrap();
    info!(target: "app", "The log file has been created.");
}
