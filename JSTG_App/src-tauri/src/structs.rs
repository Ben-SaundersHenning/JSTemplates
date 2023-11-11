use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessorListing {
    pub registration_id: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessor {
    pub registration_id: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub qualifications: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCompanyListing {
    pub unique_id: i64,
    pub common_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T> {
    pub asmt_type: String,
    pub adjuster: String,
    pub insurance_company: String,
    pub claim_number: String,
    pub date_of_assessment: String,
    pub seiden_file_number: String,
    pub referral_company: ReferralCompanyListing,
    pub assessor: AssessorListing,
    pub claimant: Claimant,
    pub asmt_specifics: T,
    pub questions: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessment<T> {
    pub asmt_type: String,
    pub adjuster: String,
    pub insurance_company: String,
    pub claim_number: String,
    pub date_of_assessment: String,
    pub seiden_file_number: String,
    pub referral_company: ReferralCompany,
    pub assessor: Assessor,
    pub claimant: Claimant,
    pub asmt_specifics: T,
    pub questions: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claimant {
    pub first_name: String,
    pub last_name: String,
    pub age: String,
    pub youth: String,
    pub address: Address,
    pub date_of_birth: String,
    pub date_of_loss: String,
    pub gender: Gender,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCompany {
    pub unique_id: i64,
    pub name: String,
    pub common_name: String,
    pub address: Address,
    pub phone: String,
    pub fax: String,
    pub email: String
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
    pub city: String,
    pub province: String,
    pub province_ab: String,
    pub postal_code: String,
    pub country: String,
    pub address_long: String

}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Gender {
    pub title: String,
    pub pronouns: Pronouns
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pronouns {
    pub p_0: String, //male-female-other
    pub p_1: String, //he-she-they
    pub p_2: String, //his-her-their
    pub p_3: String //himself-herself-themself
}
