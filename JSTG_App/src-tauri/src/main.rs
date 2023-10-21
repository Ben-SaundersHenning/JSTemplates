// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use db::get_path;

mod db;
mod request_builder;
mod structs;

fn main() {

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![double, greet, request_document, get_assessors, get_path, test, get_companies])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn double(count: i32) -> i32 {
    count * 2
}

#[tauri::command]
fn test(data: String) {
    println!("In the test method");
    let map: HashMap<&str, String> = request_builder::build_request(data);

    for (key, val) in map.iter() {
        println!("{key}: |{val}|");
    }

}

#[tauri::command]
fn get_assessors() -> Vec<structs::Assessor> {
    db::get_all_assessor_info()
}

#[tauri::command]
fn get_companies() -> Vec<structs::ReferralCompanyListing> {
    db::get_referral_company_options()
}

#[tauri::command]
async fn request_document(data: String) {

    // let mut map: HashMap<&str, &str> = serde_json::from_str(&data).unwrap();
    //
    // let template_path: String;
    // let image_path: String;
    //
    // if cfg!(windows) {
    //     template_path = get_path("Windows", "Templates");
    //     image_path = get_path("Windows", "Images");
    // }
    // else {
    //     template_path = get_path("OpenSuse", "Templates");
    //     image_path = get_path("OpenSuse", "Images");
    // };
    //
    // map.insert("TEMPLATE PATH", &template_path);
    // map.insert("IMAGE PATH", &image_path);

    let map = request_builder::build_request(data);

    let _ = send_request(map).await;

}

async fn send_request(map: HashMap<&str, String>) -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:5056/api/DocumentRequest")
        .json(&map)
        .header("responseType", "blob")
        .header("content-type", "application/json")
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("Response OK");
            let body = res.bytes().await?;

            //for development only
            let mut path: String = if cfg!(windows) {
                get_path("Windows", "Templates")
            } else {
                get_path("OpenSuse", "Templates")
            };

            path.push_str("TEST.docx");

            let mut file: File = File::create(path).unwrap();
            let _ = file.write_all(&body);

        }
        status => {
            println!("StatusCode is not okay {status}");
        }
    }

    Ok(())

}
