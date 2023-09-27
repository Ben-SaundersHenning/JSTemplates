// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
  // println!("Trying the DB function:\n\n");
  // test();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![double, greet, test])
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
async fn test(test: String) {

    println!("In test");
    println!("{test}");
    let map: HashMap<&str, &str> = serde_json::from_str(&test).unwrap();
    for key in map.keys() {
        println!("{key}");
    }
    for val in map.values() {
        println!("{val}");
    }
    // let value = serde_json::from_str(&test).unwrap();
    // println!("{value}");
    // let mut map = HashMap<String, String>;
    
    
    // let _ = send_request(json_to_hashmap(json, keys));
    println!("Going into send request");
    let _ = send_request(map).await;


}

// fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, String>> {
//     let mut lookup: HashMap<String, String> = serde_json::from_str(json).unwrap();
//     let mut map = HashMap::new();
//     for key in keys {
//         let (k, v) = lookup.remove_entry (key).unwrap();
//         map.insert(k, v);
//     }
//     Ok(map)
// }

async fn send_request(map: HashMap<&str, &str>) -> Result<(), Box<dyn std::error::Error>> {

    // let mut map = HashMap::new();
    // map.insert("baseTemplate", "file.docx");
    //
    // let mut data = HashMap::new();
    // data.insert("baseAddress", "/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/CAT.docx");
    //
    println!("In send request");
    let client = reqwest::Client::new();

    let res = client.post("http://localhost:5056/api/DocumentRequest")
        .json(&map)
        // .body(&data)
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
        }
        status => {
            println!("StatusCode is not okay {status}");
        }
    }

    Ok(())

    // let client = reqwest::Client::new();
    //
    // let request = client.post("http://localhost:5056/api/DocumentRequest")
    //     .body("{\"baseTemplate\": \"/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/CAT.docx\"")
    //     .header("responseType", "blob")
    //     .header("content-type", "application.json")
    //     .send()
    //     .await
    //     .unwrap()
    //     .bytes()
    //     .await
    //     .unwrap();
    //
    // let mut file = File::create("/run/media/ben/Windows/Users/Ben Saunders-Henning/AppData/Roaming/JSTemplates/templates/TEST.docx").unwrap();
    // file.write_all(&request).unwrap();
    

}

// fn test() {
//
//     let connection = sqlite::open("/home/ben/projects/JSTG/JSOT.db").unwrap();
//
//     let query = "
//         SELECT * FROM [Assessors];
//     ";
//
//     connection
//         .iterate(query, |pairs| {
//             for &(name, value) in pairs.iter() {
//                 println!("{} = {}", name, value.unwrap());
//             }
//             true
//         })
//         .unwrap();
//
// }
