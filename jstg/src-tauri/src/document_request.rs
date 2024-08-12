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
    referral_company_id: i16,
    date_of_assessment: NaiveDate,
    claimant: db::Claimant,
    document_id: i16,
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
    referral_company: db::ReferralCompany,
    date_of_assessment: String,
    claimant: db::Claimant,
    document: Document,
}

impl FormRequest {

    fn from_json(data: String) -> Result<Self, Error> {
        let request: FormRequest = serde_json::from_str(&data)?;
        Ok(request)
    }

    async fn build_document_request(&self) -> Result<DocumentRequest, Error> {

        // 1. Retrieive assessor
        let assessor: db::Assessor = db::get_assessor(&self.assessor_registration_id)
                                     .await
                                     .unwrap()
                                     .unwrap();

        // 2. Retrieive referral company
        let referral_company: db::ReferralCompany = db::get_referral_company(self.referral_company_id)
                                     .await
                                     .unwrap()
                                     .unwrap();

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
