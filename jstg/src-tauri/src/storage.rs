// local settings

use std::path::Path;

struct Settings {
    save_dir: String
}

#[tauri::command]
pub async fn get_settings(app_handle: tauri::AppHandle) -> Settings {

    let app_name = &app_handle.package_info().name;

    let settings_file_path = Path::new(&tauri::api::path::config_dir().unwrap())
        .join(app_name);

    // println!("Name: {}", settings_file_path.display());

    todo!()
}
