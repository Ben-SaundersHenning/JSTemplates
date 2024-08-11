// request processing
// this file may decompose to:
//
// document_request.rs
// ac.rs
// cat.rs
// mrb.rs
// neb.rs
//
// For individual processing.

use crate::db;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentRequest {
    assessor_registration_id: String,
    adjuster: String,
    insurance_company: String,
    claim_number: String,
    referral_company_id: u16,
    date_of_assessment: String,
    claimant: db::Claimant,
    document_id: u16,
}

#[tauri::command]
pub async fn request_document(data: String) {

    println!("{}", data);
    let _request: DocumentRequest = serde_json::from_str(&data).unwrap();

}
