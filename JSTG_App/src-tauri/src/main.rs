// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use db::Assessor;

mod db;

fn main() {
  // println!("Trying the DB function:\n\n");
  // test();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![double, greet, test, print_assessors])
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
fn print_assessors() -> Vec<Assessor> {
    println!("path:");
    println!("{}", db::get_path("OpenSuse", "Templates"));
    db::get_all_assessor_info()
}

#[tauri::command]
async fn test(test: String) {

    let map: HashMap<&str, &str> = serde_json::from_str(&test).unwrap();

    // for key in map.keys() {
    //     println!("Key: {key}");
    // }
    // for val in map.values() {
    //     println!("Val: {val}");
    // }

    println!("Going into send request");
    let _ = send_request(map).await;


}

async fn send_request(map: HashMap<&str, &str>) -> Result<(), Box<dyn std::error::Error>> {

    println!("In send request");
    let client = reqwest::Client::new();

    let res = client.post("http://localhost:5056/api/DocumentRequest")
        .json(&map)
        .header("responseType", "blob")
        .header("content-type", "application/json")
        .send()
        .await?;
    println!("The post has been sent");

    match res.status() {
        reqwest::StatusCode::OK => {
            println!("Response OK");
            let body = res.bytes().await?;
            let mut file = File::create("/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/TEST.docx").unwrap();
            let _ = file.write_all(&body);
            println!("Wrote to the test file");
        }
        status => {
            println!("StatusCode is not okay {status}");
        }
    }

    Ok(())

}
