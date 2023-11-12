// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;
use chrono::{NaiveDate, Datelike, Utc};
use db::get_path;

mod db;
mod request_builder;
mod structs;

fn main() {

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![double, greet, request_document, get_assessors, get_path, get_companies])
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
fn get_assessors() -> Vec<structs::AssessorListing> {
    db::get_assessor_options()
}

#[tauri::command]
fn get_companies() -> Vec<structs::ReferralCompanyListing> {
    db::get_referral_company_options()
}

#[tauri::command]
async fn request_document(data: String) {

    match request_builder::build_request(data) {
        Ok(asmt) => {
            let _ = send_request(asmt).await;
        },
        _ => {}
    };

}

async fn send_request(asmt_data: structs::Assessment<serde_json::Value>) -> Result<(), Box<dyn std::error::Error>> {

    let request = serde_json::to_string(&asmt_data).unwrap();

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:5056/api/DocumentRequest/DocRequest")
        .json(&request)
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
                get_path("Windows", "Assessments")
            } else {
                get_path("OpenSuse", "Assessments")
            };

            let date = match NaiveDate::parse_from_str(&asmt_data.date_of_assessment, "%Y-%m-%d") {
                Ok(d) => d, //return formatted date
                _ => Utc::now().naive_local().date() //try second format
            };

            let year: String = date.year().to_string();
            let month: String = match date.month() {
                1 => "January".to_string(),
                2 => "February".to_string(),
                3 => "March".to_string(),
                4 => "April".to_string(),
                5 => "May".to_string(),
                6 => "June".to_string(),
                7 => "July".to_string(),
                8 => "August".to_string(),
                9 => "September".to_string(),
                10 => "October".to_string(),
                11 => "November".to_string(),
                12 => "December".to_string(),
                _ => "Unknown".to_string(),
            };

            let ref_name = &asmt_data.referral_company.common_name;
            let client_first_name = &asmt_data.claimant.first_name;
            let client_last_name = &asmt_data.claimant.last_name;
            let assessor_first = &asmt_data.assessor.first_name;
            let assessor_last = &asmt_data.assessor.last_name;
            let assessor_initials = format!("{}{}", assessor_first.chars().next().unwrap(), assessor_last.chars().next().unwrap());
            let asmt_type = &asmt_data.asmt_type.replace(".docx", "");

            path.push_str(format!("{year}/{month}/{assessor_first}/{client_first_name} {client_last_name}/").as_str());

            match create_dir_all(path.as_str()) {
                Ok(_x) => path.push_str(format!("{ref_name}_{asmt_type}_{client_first_name} {client_last_name}_{assessor_initials}.docx").as_str()),
                _ => path.push_str("REPLACED.docx"),
            }

            let mut file: File = File::create(path).unwrap();
            let _ = file.write_all(&body);

        }
        status => {
            println!("StatusCode is not okay {status}");
        }
    }

    Ok(())

}
