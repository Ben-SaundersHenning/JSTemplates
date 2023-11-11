//this file will take in the asmtData object and convert it into the 
//key-value pairs that the API needs to generate a document. Note that
//its being done this way as practice with Rust.

use serde_json::Value;
use chrono::NaiveDate;
use crate::db;
use crate::structs::{Claimant, Assessor, Address, Gender, Pronouns, Request, ReferralCompany, Assessment};


pub fn build_request(data: String) -> Result<String, serde_json::Error> {

    /* What values need to be filled into or formatted here?
     *
     * - claimant age 
     * - claimant youth 
     * - asmtSpecificis in the future
     * - trim questions in the future
     *
     */

    let mut request: Request<Value> = serde_json::from_str(&data).unwrap();

    let mut referral_company: ReferralCompany = db::get_referral_company(request.referral_company).unwrap();
    build_long_address(&mut referral_company.address);

    let assessor: Assessor = db::get_assessor(request.assessor).unwrap();

    request.date_of_assessment = format_date(&request.date_of_assessment);
    request.claimant.date_of_loss = format_date(&request.claimant.date_of_loss);
    request.claimant.date_of_birth = format_date(&request.claimant.date_of_birth);
    build_long_address(&mut request.claimant.address);
    request.claimant.address.province = get_province_or_territory(&request.claimant.address.province_ab);
    // calculate_age(&mut request.claimant);

    //these cant be constants because of they have to be Strings (because
    //of the serialization), but they cant be &str, because thats not possible
    //with const
    match request.claimant.gender.pronouns.p_0.as_str() {
        "male" => { request.claimant.gender =
            Gender {
                title: String::from("Mr"),
                pronouns: Pronouns {
                    p_0: String::from("male"),
                    p_1: String::from("he"),
                    p_2: String::from("his"),
                    p_3: String::from("himself")
                }
            };
        },
        "female" => { request.claimant.gender =
            Gender {
                title: String::from("Ms"),
                pronouns: Pronouns {
                    p_0: String::from("female"),
                    p_1: String::from("she"),
                    p_2: String::from("her"),
                    p_3: String::from("herself")
                }
            };
        },
        _ => { request.claimant.gender =
            Gender {
                title: String::from("Mx"),
                pronouns: Pronouns {
                    p_0: String::from("{other}"),
                    p_1: String::from("they"),
                    p_2: String::from("their"),
                    p_3: String::from("themself")
                }
            };
        }
    };

    let assesment = Assessment {
        asmt_type: request.asmt_type,
        adjuster: request.adjuster,
        insurance_company: request.insurance_company,
        claim_number: request.claim_number,
        date_of_assessment: request.date_of_assessment,
        seiden_file_number: request.seiden_file_number,
        referral_company,
        assessor,
        claimant: request.claimant,
        asmt_specifics: request.asmt_specifics,
        questions: request.questions
    };

    serde_json::to_string(&assesment)

}

fn build_long_address(address: &mut Address) {
    address.address_long = format!("{}, {} {}, {}",
        address.address,
        address.city,
        address.province_ab,
        address.postal_code);
}

fn get_province_or_territory(province_or_territory: &str) -> String {

    match province_or_territory {
        "AB" => "Alberta".to_string(),
        "BC" => "British Columbia".to_string(),
        "MB" => "Manitoba".to_string(),
        "NB" => "New Brunswick".to_string(),
        "NL" => "Newfoundland and Labrador".to_string(),
        "NS" => "Nova Scotia".to_string(),
        "ON" => "Ontaio".to_string(),
        "PEI" => "Prince Edward Island".to_string(),
        "QC" => "Quebec".to_string(),
        "SK" => "Saskatchewan".to_string(),
        "YT" => "Yukon".to_string(),
        "NU" => "Nunavut".to_string(),
        "NT" => "Northwest Territories".to_string(),
        _ => "NULL_PROVINCE".to_string()
    }

}

// fn calculate_age(claimant: &mut Claimant) {
//     let mut age: i8 = 0;
//
//     let dob = chrono::NaiveDate::from_str(claimant.date_of_birth, 
//
// }

/*
pub fn build_request(data: String) -> HashMap<&'static str, String> {

    let request: Request<String> = serde_json::from_str(&data).unwrap();

    let referral_company: ReferralCompany = db::get_referral_company(request.referral_company).unwrap();

    let assessor: Assessor = db::get_assessor(request.assessor).unwrap();
    let assessor_first: String = assessor.first_name.clone();

    let mut map: HashMap<&str, String> = HashMap::from([
        ("ADJUSTER", request.adjuster),
        ("INSURANCE COMPANY", request.insurance_company),
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
*/

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
