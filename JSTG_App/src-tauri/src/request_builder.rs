//this file will take in the asmtData object and convert it into the 
//key-value pairs that the API needs to generate a document. Note that
//its being done this way as practice with Rust.

use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::db;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    adjuster: String,
    ins_company: String,
    claim_number: String,
    date_of_assessment: String,
    referral_company: String,
    asmt_type: String,
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
    salutation: String,
    age: String,
    gender: String,
    address_long: String,
    date_of_birth: String,
    date_of_loss: String
}

pub fn build_request(data: String) -> HashMap<&'static str, String> {

    let _v: Value = serde_json::from_str(&data).unwrap();

    let request: Request = serde_json::from_str(&data).unwrap();

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
        ("TEMPLATE", request.asmt_type),

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
        map.insert("CLIENT SALUTATION", "Mr".to_string());
    } else if request.claimant.gender == "female" {
        map.insert("HE---SHE_Lower", "she".to_string());
        map.insert("HE---SHE_Upper", "She".to_string());
        map.insert("MALE---FEMALE_Lower", "female".to_string());
        map.insert("HIS---HER_Lower", "her".to_string());
        map.insert("HIM---HER_Lower", "her".to_string());
        map.insert("CLIENT SALUTATION", "Ms".to_string());
    }


    if request.therapist.first_name == "Joan" {
        map.insert("IMAGE", "JS.png".to_string());
    } else if request.therapist.first_name == "Montana" {
        map.insert("IMAGE", "MM.png".to_string());
    } else if request.therapist.first_name == "Anghela" {
        map.insert("IMAGE", "AS.png".to_string());
    } else if request.therapist.first_name == "Josh" {
        map.insert("IMAGE", "JM.png".to_string());
    }

    //these are temporary
    let template_path: String;
    let image_path: String;

    if cfg!(windows) {
        template_path = db::get_path("Windows", "Templates");
        image_path = db::get_path("Windows", "Images");
    }
    else {
        template_path = db::get_path("OpenSuse", "Templates");
        image_path = db::get_path("OpenSuse", "Images");
    };

    map.insert("TEMPLATE PATH", template_path);
    map.insert("IMAGE PATH", image_path);

    map

}
