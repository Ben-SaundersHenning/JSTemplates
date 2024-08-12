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
use crate::Error;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormRequest {
    assessor_registration_id: String,
    adjuster: Option<String>,
    insurance_company: String,
    claim_number: String,
    referral_company_id: u16,
    date_of_assessment: NaiveDate,
    claimant: db::Claimant,
    document_id: u16,
}

#[derive(Serialize)]
struct Assessor {
    registration_id: String,
    first_name: String,
    last_last: String,
    gender: db::Gender,
    email: String,
    qualificatons_paragraph: String,
}

#[derive(Serialize)]
struct ReferralCompany {
    id: String,
    name: String,
    address: db::Address,
    phone: String,
    fax: String,
    email: String,
}

#[derive(Serialize)]
struct Document {
    id: String,
    file: String,
}

#[derive(Serialize)]
struct DocumentRequest {
    assessor: Assessor,
    adjuster: String,
    insurance_company: String,
    claim_number: String,
    referral_company: ReferralCompany,
    date_of_assessment: String,
    claimant: db::Claimant,
    document: Document,
}

impl FormRequest {

    fn from_json(data: String) -> Result<Self, Error> {
        let request: FormRequest = serde_json::from_str(&data)?;
        Ok(request)
    }

    fn build_document_request(&self) -> DocumentRequest {

        // 1. Retrieive assessor
        // 2. Retrieive referral company
        // 3. Retrieive document info
        // 4. Calculate claimant age
        // 5. Work on AC, CAT, MRB, NEB portions
        // 6. Return a Document Request

        todo!();
    }

}

#[tauri::command]
pub async fn request_document(data: String) {

    let _request = FormRequest::from_json(data).unwrap();

}
