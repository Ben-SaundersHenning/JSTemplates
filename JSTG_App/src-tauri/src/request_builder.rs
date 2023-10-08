//this file will take in the asmtData object and convert it into the 
//key-value pairs that the API needs to generate a document. Note that
//its being done this way as practice with Rust.

//Need to build a hashmap with the following keys:
//
//     "ADJUSTER"
//     "INSURANCE COMPANY"
//     "OCCUPATIONAL THERAPIST"
//     "ASSESSOR QUALS"
//
//     "CLIENT SALUTATION"
//     "CLIENT FIRST"
//     "CLIENT LAST"
//     "CLIENT AGE"
//     "CLIENT ADDRESS"
//     "DOB"
//
//     "HE---SHE_Lower"
//     "MALE---FEMALE_Lower"
//     "HIS---HER_Lower"
//     "MALE---FEMALE_LOWER"
//     "HE---SHE_Upper"
//     "HIM---HER_Lower"
//
//     "CLAIM NUMBER"
//     "DOL"
//     "DOA"
//     "REFERRAL COMPANY"
//
//     "TEMPLATE"
//     "IMAGE"
//
//     These are to be removed
//     "TEMPLATE PATH"
//     "IMAGE PATH"
//
//     Plus any asmt specific keys

use serde_json::Value;
use std::collections::HashMap;

pub fn build_request(request: String) -> HashMap<&'static str, String> {

    let v: Value = serde_json::from_str(&request).unwrap();

    let mut map: HashMap<&str, String> = HashMap::from([
        ("ADJUSTER", v["adjuster"].to_string()),
        ("INSURANCE COMPANY", v["insCompany"].to_string()),
        ("OCCUPATIONAL THERAPIST", 
            v["therapist"]["salutation"].to_string()
            + ". " 
            + &v["therapist"]["first_name"].to_string()
            + " " 
            + &v["therapist"]["last_name"].to_string()),
        ("ASSESSOR QUALS", v["therapist"]["qualifications"].to_string()),
        ("CLIENT FIRST", v["claimant"]["firstName"].to_string()),
        ("CLIENT LAST", v["claimant"]["lastName"].to_string()),
        ("CLIENT AGE", v["claimant"]["age"].to_string()),
        ("CLIENT ADDRESS", v["claimant"]["addressLong"].to_string()),
        ("DOB", v["claimant"]["dateOfBirth"].to_string()),
        ("DOL", v["claimant"]["dateOfLoss"].to_string()),
        ("CLAIM NUMBER", v["claimNumber"].to_string()),
        ("DOA", v["dateOfAssessment"].to_string()),
        ("REFERRAL COMPANY", v["referralCompany"].to_string()),

        // ("TEMPLATE", v[""].to_string()),
        // ("IMAGE", v[""].to_string()),
        // // ("MALE---FEMALE_LOWER", v[""].to_string()),
    ]);

    if v["claimant"]["gender"] == "male" {
        map.insert("HE---SHE_Lower", "he".to_string());
        map.insert("HE---SHE_Upper", "He".to_string());
        map.insert("MALE---FEMALE_Lower", "male".to_string());
        map.insert("HIS---HER_Lower", "his".to_string());
        map.insert("HIM---HER_Lower", "him".to_string());
        map.insert("CLIENT SALUATION", "Mr".to_string());
    } else {
        map.insert("HE---SHE_Lower", "she".to_string());
        map.insert("HE---SHE_Upper", "She".to_string());
        map.insert("MALE---FEMALE_Lower", "female".to_string());
        map.insert("HIS---HER_Lower", "her".to_string());
        map.insert("HIM---HER_Lower", "her".to_string());
        map.insert("CLIENT SALUATION", "Ms".to_string());
    }

    map

}
