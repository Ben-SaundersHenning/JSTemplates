//this file will take in the asmtData object and convert it into the 
//key-value pairs that the API needs to generate a document. Note that
//its being done this way as practice with Rust.

use serde_json::Value;
use std::collections::HashMap;
use chrono::NaiveDate;
use crate::db;
use crate::structs::{Assessor, Request, ReferralCompany};


pub fn build_request(data: String) -> HashMap<&'static str, String> {

    let _v: Value = serde_json::from_str(&data).unwrap();

    let request: Request = serde_json::from_str(&data).unwrap();

    let referral_company: ReferralCompany = db::get_referral_company(request.referral_company).unwrap();

    let assessor: Assessor = db::get_assessor(request.assessor).unwrap();
    let assessor_first: String = assessor.first_name.clone();

    let mut map: HashMap<&str, String> = HashMap::from([
        ("ADJUSTER", request.adjuster),
        ("INSURANCE COMPANY", request.ins_company),
        ("CLIENT FIRST", request.claimant.first_name),
        ("CLIENT LAST", request.claimant.last_name),
        ("CLIENT AGE", request.claimant.age),
        ("CLIENT ADDRESS", request.claimant.address_long),
        ("CLAIM NUMBER", request.claim_number),
        ("DOB", format_date(&request.claimant.date_of_birth)),
        ("DOL", format_date(&request.claimant.date_of_loss)),
        ("DOA", format_date(&request.date_of_assessment)),
        ("ASSESSOR REGISTRATIONID", assessor.registration_id),
        ("ASSESSOR FIRST", assessor.first_name),
        ("ASSESSOR LAST", assessor.last_name),
        ("ASSESSOR SALUTATION", assessor.salutation),
        ("ASSESSOR EMAIL", assessor.email),
        ("ASSESSOR QUALIFICATIONS", assessor.qualifications),
        ("REFCOMP NAME", referral_company.name),
        ("REFCOMP COMMONNAME", referral_company.common_name),
        ("REFCOMP ADDRESS", referral_company.address),
        ("REFCOMP CITY", referral_company.city),
        ("REFCOMP PROVINCE", referral_company.province),
        ("REFCOMP PROVINCEAB", referral_company.province_ab),
        ("REFCOMP POSTALCODE", referral_company.postal_code),
        ("REFCOMP PHONE", referral_company.phone),
        ("REFCOMP FAX", referral_company.fax),
        ("REFCOMP EMAIL", referral_company.email),
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

    //temporary.
    match assessor_first.as_str() {
        "Joan" => map.insert("IMAGE", "JS.png".to_string()),
        "Montana" => map.insert("IMAGE", "MM.png".to_string()),
        "Anghela" => map.insert("IMAGE", "AS.png".to_string()),
        "Josh" => map.insert("IMAGE", "JM.png".to_string()),
        _ => map.insert("IMAGE", "JS.png".to_string())
    };

    //these are temporary
    let template_path: String;
    let image_path: String;

    if cfg!(windows) {
        template_path = db::get_path("Windows", "TemplatesDev");
        image_path = db::get_path("Windows", "Images");
    }
    else {
        template_path = db::get_path("OpenSuse", "TemplatesDev");
        image_path = db::get_path("OpenSuse", "Images");
    };

    map.insert("TEMPLATE PATH", template_path);
    map.insert("IMAGE PATH", image_path);

    map

}

//Intended to format a date, so that it can be parsed into a 
//dotnet DateTime object.
fn format_date(input: &str) -> String {

    //try to parse from a date like "2023-11-01"
    let date = NaiveDate::parse_from_str(input, "%Y-%m-%d");

    match date {
        Ok(d) => return d.format("%F").to_string(), //return formatted date
        _ => {} //try second format
    }

    //try to parse from a date like November 1, 2023
    let date = NaiveDate::parse_from_str(input, "%B %d, %Y");

    match date {
        Ok(d) => return d.format("%F").to_string(), //return formatted date
        _ => return input.to_string() //return input
    }

}
