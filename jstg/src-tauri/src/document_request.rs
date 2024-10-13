mod ac;
mod cat;
mod mrb;

use crate::db;
use crate::Error;
use ac::Ac;
use cat::Cat;
use mrb::Mrb;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FormRequest {
    assessor_registration_id: String,
    adjuster: Option<String>,
    insurance_company: String,
    claim_number: String,
    referral_company_id: i32,
    date_of_assessment: NaiveDate,
    claimant: db::Claimant,
    document_id: i32,
    ac: Option<Ac>,
    cat: Option<Cat>,
    mrb: Option<Mrb>,
}

impl FormRequest {

    fn from_json(data: String) -> Result<Self, Error> {
        let request: FormRequest = serde_json::from_str(&data)?;
        Ok(request)
    }

    // Builds a DocumentRequest object by consuming self.
    // TODO: Add some checks to the data returning from the DB.
    // Since the IDs are retrieved from the DB, they should theoretically
    // never be incorrect, but the DB itself could be down.
    async fn build_document_request(self) -> DocumentRequest {

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

        // 3. Retrieive document path
        let document: db::Document = db::get_document(self.document_id)
                                     .await
                                     .unwrap()
                                     .unwrap();

        // let mut ac: Ac = self.ac.clone().take();
        let ac: Option<Ac> = match &self.ac {
            Some(val) => {
                Some(Ac {
                        first_assessment: val.first_assessment,
                        date_of_last_assessment: val.date_of_last_assessment,
                        monthly_allowance: val.monthly_allowance.clone(),
                        hourly_rates: Ac::determine_hourly_rates(val.date_of_last_assessment),
                })
            },
            None => {
                None
            }
        };

        // 4. Return a Document Request
        let document_request = DocumentRequest::from_form_request(self, assessor, referral_company, document, ac);

        document_request

        // TODO
        // add hourly rates to ac

    }

}

#[derive(Serialize, Debug)]
struct DocumentRequest {
    assessor: db::Assessor,
    adjuster: Option<String>,
    insurance_company: String,
    claim_number: String,
    referral_company: db::ReferralCompany,
    date_of_assessment: NaiveDate,
    claimant: db::Claimant,
    document: db::Document,
    ac: Option<Ac>,
    cat: Option<Cat>,
    mrb: Option<Mrb>,
}

impl DocumentRequest {

    fn from_form_request(form_request: FormRequest, assessor: db::Assessor,
                         referral_company: db::ReferralCompany, document: db::Document, ac: Option<Ac>) -> Self {

        // Calculate age in years
        let doa: i32 = form_request.date_of_assessment.format("%Y%m%d").to_string().parse().unwrap();
        let dob: i32 = form_request.claimant.date_of_birth.format("%Y%m%d").to_string().parse().unwrap();
        let age = (doa - dob)/10000;

        let youth: bool = age >= 18;

        DocumentRequest {
            assessor,
            adjuster: form_request.adjuster,
            insurance_company: form_request.insurance_company,
            claim_number: form_request.claim_number,
            referral_company,
            date_of_assessment: form_request.date_of_assessment,
            claimant: db::Claimant {
                first_name: form_request.claimant.first_name, 
                last_name: form_request.claimant.last_name, 
                gender: form_request.claimant.gender, 
                age: Some(age),
                youth: Some(youth),
                date_of_birth: form_request.claimant.date_of_birth, 
                date_of_loss: form_request.claimant.date_of_loss, 
                address: form_request.claimant.address, 
            },
            document,
            ac,
            cat: form_request.cat,
            mrb: form_request.mrb,
        }

    }

}


#[tauri::command]
pub async fn request_document(data: String) {

    let request = FormRequest::from_json(data).unwrap();
    let document_request = request.build_document_request().await;
    println!("{:?}", document_request);

}
