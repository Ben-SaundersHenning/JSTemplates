use serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessor {
    pub salutation: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCompanyListing {
    pub unique_id: i64,
    pub common_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub adjuster: String,
    pub ins_company: String,
    pub claim_number: String,
    pub date_of_assessment: String,
    pub referral_company: ReferralCompanyListing,
    pub asmt_type: String,
    pub therapist: Therapist,
    pub claimant: Claimant
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Therapist {
    pub salutation: String,
    pub first_name: String,
    pub last_name: String,
    // qualifications: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    pub first_name: String,
    pub last_name: String,
    pub salutation: String,
    pub age: String,
    pub gender: String,
    pub address_long: String,
    pub date_of_birth: String,
    pub date_of_loss: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCompany {
    pub name: String,
    pub address: String,
    pub city: String,
    pub province: String,
    pub province_ab: String,
    pub postal_code: String,
    pub phone: String,
    pub fax: String,
    pub email: String
}
