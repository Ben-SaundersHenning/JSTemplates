use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Document {
    pub id: i32,
    pub common_name: String,
    pub file: String
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AssessorListing {
    pub registration_id: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Assessor {
    pub registration_id: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub qualifications_paragraph: String
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ReferralCompanyListing {
    pub id: i32,
    pub common_name: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ReferralCompany {
    pub name: String,
    pub common_name: String,
    pub phone: String,
    pub fax: String,
    pub email: String,
    #[sqlx(flatten)]
    pub address: Address,
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub province: String,
    pub province_abbreviated: String,
    pub postal_code: String,
    pub country: String,
    pub address_long: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct Gender {
    pub title: String,
    pub pronouns: Pronouns
}

#[derive(sqlx::FromRow)]
pub struct Path {
    pub path: String,
}

//The upper and lowers are a temporary solution.
#[derive(Serialize, Deserialize, Default)]
pub struct Pronouns {
    pub p0_lower: String, //male-female-other
    pub p0_upper: String, //male-female-other
    pub p1_lower: String, //he-she-they
    pub p1_upper: String, //he-she-they
    pub p2_lower: String, //his-her-their
    pub p2_upper: String, //his-her-their
    pub p3_lower: String, //himself-herself-themself
    pub p3_upper: String //himself-herself-themself
}

#[derive(Serialize, Deserialize, Default)]
pub struct Ac {
    pub first_assessment: bool,
    pub date_of_last_assessment: String,
    pub current_monthly_allowance: String,
    pub hourly_rates: Vec<String>
}

#[derive(Serialize, Deserialize, Default)]
// #[serde(rename_all = "camelCase")]
pub struct Specifics {
    pub ac: Ac
}
