// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

mod db;
mod document_request;
mod storage;

fn main() {

    tauri::Builder::default()
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {

    let mut app_logs: String = (&app.package_info().name).into();
    app_logs.push_str("/logs");

    let log_file_path = Path::new(&tauri::api::path::config_dir().unwrap())
        .join(app_logs);

    // println!("Logs: {}", log_file_path.display());

    Ok(())

}

