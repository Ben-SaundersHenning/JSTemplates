// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::fs::create_dir_all;
use std::io::Write;
use chrono::{NaiveDate, Datelike, Utc};
use request_builder::build_request;
use std::error::Error;
use log4rs;
use log::info;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

mod db;
mod settings;
mod request_builder;
mod structs;

fn main() {

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}")))
        .build("log/app.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("app", LevelFilter::Debug))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();

    info!(target: "app", "Tauri App is opening.");

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        request_document,
        get_assessors,
        get_path,
        get_document_options,
        get_referral_company_options
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

#[tauri::command]
async fn get_assessors() -> Result<Vec<structs::AssessorListing>, String> {
    match db::get_assessor_options().await {
        Ok(val) => Ok(val),
        _ => Err("Unable to retrieve assessor options.".to_string())
    }
}

#[tauri::command]
async fn get_referral_company_options() -> Result<Vec<structs::ReferralCompanyListing>, String> {
    match db::get_referral_company_options().await {
        Ok(val) => Ok(val),
        _ => Err("Unable to retrieve referral company options.".to_string())
    }
}

#[tauri::command]
async fn get_document_options() -> Result<Vec<structs::Document>, String> {
    match db::get_document_options().await {
        Ok(val) => Ok(val),
        _ => Err("Unable to retrieive the available templates.".to_string())
    }
}

#[tauri::command]
async fn get_path(system: &str, dir: &str) -> Result<String, String> {
    match db::get_path(system, dir).await {
        Ok(val) => Ok(val),
        _ => Err("Error: unable to find path.".to_string())
    }
}

// //test method
// #[tauri::command]
// async fn print_request(data: String) {
//     match build_request(data).await {
//         Ok(asmt) => {
//             let request = serde_json::to_string(&asmt).unwrap();
//             println!("REQUEST:\n\n{0}\n\n", request);
//         },
//         _ => {}
//     };
// }

#[tauri::command]
async fn request_document(data: String, handle: tauri::AppHandle) {

    info!(target: "app", "A document request has been received.");

    match build_request(data).await {
        Ok(asmt) => {
            send_request(asmt, handle).await;
        },
        _ => {}
    };

}

async fn submit_request(asmt_data: &structs::Assessment<serde_json::Value>, is_f1: bool, endpoint: &str, handle: tauri::AppHandle) -> Result<(), Box<dyn Error>> {

    let request = serde_json::to_string(&asmt_data).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(endpoint)
        .json(&request)
        .header("responseType", "blob")
        .header("content-type", "application/json")
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            info!(target: "app", "Response OK from Document API");
            let body = res.bytes().await?;

            //for development only
            // let mut path: String = if cfg!(windows) {
            //     get_path("Windows", "Assessments").await?
            // } else {
            //     get_path("OpenSuse", "Assessments").await?
            // };

            let mut path: String = settings::get_save_dir(handle).await.to_string();

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
            let _asmt_type = &asmt_data.asmt_type.replace(".docx", "");
            let asmt_type: &str = match is_f1 {
                true => "F1",
                false => _asmt_type
            };

            path.push_str(format!("{year}/{month}/{assessor_first}/{client_first_name} {client_last_name}/").as_str());

            match create_dir_all(path.as_str()) {
                Ok(_x) => path.push_str(format!("{ref_name}_{asmt_type}_{client_first_name} {client_last_name}_{assessor_initials}.docx").as_str()),
                _ => path.push_str("REPLACED.docx"),
            }

            // info!(target: "app", format!("Saving a file to the path: {0}", path));
            info!(target: "app", "Saving a file to the path: {0}", path);

            let mut file: File = File::create(path).unwrap();
            let _ = file.write_all(&body);

        }
        _status => {
            info!(target: "app", "Response not OK from Document API");
        }
    }

    Ok(())

}

async fn send_request(asmt_data: structs::Assessment<serde_json::Value>, handle: tauri::AppHandle) {

    if asmt_data.asmt_type.contains("AC") || asmt_data.asmt_type.contains("F1") {
        let _ = submit_request(&asmt_data, true, "http://localhost:5056/api/DocumentRequest/F1Request", handle.clone()).await;
    }

    if !asmt_data.asmt_type.contains("F1") {
        let _ = submit_request(&asmt_data, false, "http://localhost:5056/api/DocumentRequest/DocRequest", handle.clone()).await;
    }

}
