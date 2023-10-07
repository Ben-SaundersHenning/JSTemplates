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
//
//     Plus any asmt specific keys

use serde_json::Value;
use std::collections::HashMap;

pub fn build_request(request: String) -> HashMap<String, String> {

    let _v: Value = serde_json::from_str(&request).unwrap();

    let temp: HashMap<String, String> = HashMap::new();

    temp

}
