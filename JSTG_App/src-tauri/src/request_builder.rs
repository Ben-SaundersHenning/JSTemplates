//this file will take in the asmtData object and convert it into the 
//key-value pairs that the API needs to generate a document. Note that
//its being done this way as practice with Rust.

use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    adjuster: String,
    ins_company: String,
    claim_number: String,
    date_of_assessment: String,
    referral_company: String,
    therapist: Therapist,
    claimant: Claimant
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Therapist {
    salutation: String,
    first_name: String,
    last_name: String,
    // qualifications: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claimant {
    first_name: String,
    last_name: String,
    age: String,
    gender: String,
    address_long: String,
    date_of_birth: String,
    date_of_loss: String
}

pub fn build_request(obj: String) -> HashMap<&'static str, String> {

    let _v: Value = serde_json::from_str(&obj).unwrap();

    let request: Request = serde_json::from_str(&obj).unwrap();

    let mut map: HashMap<&str, String> = HashMap::from([
        ("ADJUSTER", request.adjuster),
        ("INSURANCE COMPANY", request.ins_company),
        ("OCCUPATIONAL THERAPIST",
            request.therapist.salutation
            + ". "
            + &request.therapist.first_name
            + " "
            + &request.therapist.last_name),
        // ("ASSESSOR QUALS", request.therapist.qualifications),
        ("CLIENT FIRST", request.claimant.first_name),
        ("CLIENT LAST", request.claimant.last_name),
        ("CLIENT AGE", request.claimant.age),
        ("CLIENT ADDRESS", request.claimant.address_long),
        ("DOB", request.claimant.date_of_birth),
        ("DOL", request.claimant.date_of_loss),
        ("CLAIM NUMBER", request.claim_number),
        ("DOA", request.date_of_assessment),
        ("REFERRAL COMPANY", request.referral_company),

        // ("TEMPLATE", v[""].to_string()),
        // ("IMAGE", v[""].to_string()),
        // // ("MALE---FEMALE_LOWER", v[""].to_string()),
    ]);

    if request.claimant.gender == "male" {
        map.insert("HE---SHE_Lower", "he".to_string());
        map.insert("HE---SHE_Upper", "He".to_string());
        map.insert("MALE---FEMALE_Lower", "male".to_string());
        map.insert("HIS---HER_Lower", "his".to_string());
        map.insert("HIM---HER_Lower", "him".to_string());
        map.insert("CLIENT SALUATION", "Mr".to_string());
    } else if request.claimant.gender == "female" {
        map.insert("HE---SHE_Lower", "she".to_string());
        map.insert("HE---SHE_Upper", "She".to_string());
        map.insert("MALE---FEMALE_Lower", "female".to_string());
        map.insert("HIS---HER_Lower", "her".to_string());
        map.insert("HIM---HER_Lower", "her".to_string());
        map.insert("CLIENT SALUATION", "Ms".to_string());
    }

    map

}
