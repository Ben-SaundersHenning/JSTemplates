//this file will take in the asmtData object and convert it into the 
//key-value pairs that the API needs to generate a document. Note that
//its being done this way as practice with Rust.

use std::error::Error;

use serde_json::Value;
use chrono::{NaiveDate, Datelike};
use crate::db;
use crate::structs::{Claimant, Assessor, Address, Gender, Request, ReferralCompany, Assessment, Ac, Cat, Mrb};
use log::info;

const DATE0: &str = "2008-03-31";
const DATE1: &str = "2010-09-01";
const DATE2: &str = "2014-06-01";
const DATE3: &str = "2015-10-01";
const DATE4: &str = "2016-10-01";
const DATE5: &str = "2017-10-01";
const DATE6: &str = "2018-01-01";
const DATE7: &str = "2018-04-14";

fn build_ac(data: &Ac) -> Value {

    let mut ac: Ac = data.clone();

    ac.date_of_last_assessment = format_date(&ac.date_of_last_assessment);

    match parse_date(&ac.date_of_last_assessment) {
        Some(val) => {
            if val >= parse_date(DATE0).unwrap()
                && val < parse_date(DATE1).unwrap() {

                    ac.hourly_rates.push(String::from("11.23"));
                    ac.hourly_rates.push(String::from("8.75"));
                    ac.hourly_rates.push(String::from("17.98"));

            } else if val >= parse_date(DATE1).unwrap()
                && val < parse_date(DATE2).unwrap() {

                    ac.hourly_rates.push(String::from("13.19"));
                    ac.hourly_rates.push(String::from("10.25"));
                    ac.hourly_rates.push(String::from("19.35"));

            } else if val >= parse_date(DATE2).unwrap()
                && val < parse_date(DATE3).unwrap() {

                    ac.hourly_rates.push(String::from("13.19"));
                    ac.hourly_rates.push(String::from("11.00"));
                    ac.hourly_rates.push(String::from("19.35"));

            } else if val >= parse_date(DATE3).unwrap()
                && val < parse_date(DATE4).unwrap() {

                    ac.hourly_rates.push(String::from("13.19"));
                    ac.hourly_rates.push(String::from("11.25"));
                    ac.hourly_rates.push(String::from("19.35"));

            } else if val >= parse_date(DATE4).unwrap()
                && val < parse_date(DATE5).unwrap() {

                    ac.hourly_rates.push(String::from("14.90"));
                    ac.hourly_rates.push(String::from("11.40"));
                    ac.hourly_rates.push(String::from("21.11"));

            } else if val >= parse_date(DATE5).unwrap()
                && val < parse_date(DATE6).unwrap() {

                    ac.hourly_rates.push(String::from("14.90"));
                    ac.hourly_rates.push(String::from("11.60"));
                    ac.hourly_rates.push(String::from("21.11"));

            } else if val >= parse_date(DATE6).unwrap()
                && val < parse_date(DATE7).unwrap() {

                    ac.hourly_rates.push(String::from("14.90"));
                    ac.hourly_rates.push(String::from("14.00"));
                    ac.hourly_rates.push(String::from("21.11"));

            } else {

                    ac.hourly_rates.push(String::from("14.90"));
                    ac.hourly_rates.push(String::from("14.00"));
                    ac.hourly_rates.push(String::from("21.11"));

            }
        },
        None => {
            ac.hourly_rates.push(String::from("14.90"));
            ac.hourly_rates.push(String::from("14.00"));
            ac.hourly_rates.push(String::from("21.11"));
        } //unable to parse date, use defaults.
    };

    if ac.first_assessment {
        ac.current_monthly_allowance = String::from("");
        ac.date_of_last_assessment = String::from("");
    }

    return serde_json::to_value(ac).unwrap();

}

fn build_cat(data: &Cat) -> Value {

    let mut cat: Cat = data.clone();

    cat.date = format_date(&cat.date);

    return serde_json::to_value(cat).unwrap();

}

fn build_mrb(data: &Mrb) -> Value {

    let mut mrb: Mrb = data.clone();

    mrb.date = format_date(&mrb.date);

    return serde_json::to_value(mrb).unwrap();

}

//Value may be the best option here because the data object is 
//always completely dynamic - it's fields are never guaranteed.
//They are only known by whats in the string parameter.
//Another options could be to always send data with every possible value,
//and trim down based on the string parameter.
fn build_types_data(data: &Value, types: &Vec<String>) -> Value {

    let mut val: Value = data.clone();

    for asmt_type in types {

        //Note that this causes a repetition when the type
        //includes AC - since any AC is an F1. This needs to
        //be changed in the future.
        match asmt_type.as_str() {
            "AC" => {
                let ac_clone: Ac = serde_json::from_value(data["ac"].clone()).unwrap();
                val["ac"] = build_ac(&ac_clone);
            },
            "F1" => {
                let ac_clone: Ac = serde_json::from_value(data["ac"].clone()).unwrap();
                val["ac"] = build_ac(&ac_clone);
            },
            "CAT" => {
                let cat_clone: Cat = serde_json::from_value(data["cat"].clone()).unwrap();
                val["cat"] = build_cat(&cat_clone);
            },
            "CAT_GOSE" => {},
            "MRB" => {
                let mrb_clone: Mrb = serde_json::from_value(data["mrb"].clone()).unwrap();
                val["mrb"] = build_mrb(&mrb_clone);
            },
            "NEB" => {},
            _ => {}

        };
    }

    val

}

pub async fn build_request(data: String) -> Result<Assessment<Value>, Box<dyn Error + Send + Sync>> {

    let mut request: Request<Value> = serde_json::from_str(&data).unwrap();

    let mut referral_company: ReferralCompany = match db::get_referral_company(request.referral_company).await {
        Ok(val) => val.unwrap(),
        Err(_e) => {return Err("Unable to retrieve the referral company")?;}
    };

    build_long_address(&mut referral_company.address);

    let assessor: Assessor = match db::get_assessor(request.assessor).await {
        Ok(val) => val.unwrap(),
        Err(_e) => {return Err("Unable to retrieve the assessor")?;}
    };

    request.date_of_assessment = format_date(&request.date_of_assessment);
    request.claimant.date_of_loss = format_date(&request.claimant.date_of_loss);
    request.claimant.date_of_birth = format_date(&request.claimant.date_of_birth);
    request.claimant.address.province = get_province_or_territory(&request.claimant.address.province_abbreviated);
    build_long_address(&mut request.claimant.address);
    calculate_age(&mut request.claimant);
    set_gender_values(&mut request.claimant.gender);

    request.types = request.asmt_type.replace(".docx", "")
                                     .split(' ')
                                     .map(|s| s.to_string())
                                     .collect();

    request.asmt_specifics = build_types_data(&request.asmt_specifics, &request.types);

    let assesment = Assessment {
        asmt_type: request.asmt_type,
        types: request.types,
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

    info!(target: "app", "The request has been built succesfully.");

    Ok(assesment)

}

fn build_long_address(address: &mut Address) {
    address.address_long = format!("{}, {} {}, {}",
        address.address,
        address.city,
        address.province_abbreviated,
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

fn calculate_age(claimant: &mut Claimant) -> bool {

    let mut age: i32;

    let _dob = chrono::NaiveDate::parse_from_str(&claimant.date_of_birth, "%F");

    let dob = match _dob {
        Ok(val) => {
            val
        },
        _ => {return false;}
    };

    let now = chrono::Local::now();

    //assume the b-day has passed this year
    age = now.year() - dob.year();

    if now.month() < dob.month() { //b-day cant have passed
        age = age - 1;
    } else if now.month() == dob.month() { //b-day may have passed
        if now.day() < dob.day() { //b-day cant have passed
            age = age - 1;
        }
    }

    claimant.age = age.to_string();

    if age < 18 {
        claimant.youth = String::from("true");
    }

    true

}

fn set_gender_values(gender: &mut Gender) {

    //need the vals to be String type, so const clones wont work.
    match gender.pronouns.p0_lower.as_str() {
        "male" => {
            gender.title = String::from("Mr");
            gender.pronouns.p0_lower = String::from("male");
            gender.pronouns.p1_lower = String::from("he");
            gender.pronouns.p2_lower = String::from("his");
            gender.pronouns.p3_lower = String::from("himself");
            gender.pronouns.p0_upper = String::from("Male");
            gender.pronouns.p1_upper = String::from("He");
            gender.pronouns.p2_upper = String::from("His");
            gender.pronouns.p3_upper = String::from("Himself");
        },
        "female" => {
            gender.title = String::from("Ms");
            gender.pronouns.p0_lower = String::from("female");
            gender.pronouns.p1_lower = String::from("she");
            gender.pronouns.p2_lower = String::from("her");
            gender.pronouns.p3_lower = String::from("herself");
            gender.pronouns.p0_upper = String::from("Female");
            gender.pronouns.p1_upper = String::from("She");
            gender.pronouns.p2_upper = String::from("Her");
            gender.pronouns.p3_upper = String::from("Herself");
        },
        _ => {
            gender.title = String::from("Mx");
            gender.pronouns.p0_lower = String::from("{other}");
            gender.pronouns.p1_lower = String::from("they");
            gender.pronouns.p2_lower = String::from("their");
            gender.pronouns.p3_lower = String::from("themself");
            gender.pronouns.p0_upper = String::from("{Other}");
            gender.pronouns.p1_upper = String::from("They");
            gender.pronouns.p2_upper = String::from("Their");
            gender.pronouns.p3_upper = String::from("Themself");
        }
    };

}

//Intended to format a date, so that it can be parsed into a 
//dotnet DateTime object.
fn format_date(input: &str) -> String {

    match parse_date(input) {
        Some(val) => {return val.format("%F").to_string()},
        None => {return input.to_string()}
    };

}

fn parse_date(input: &str) -> Option<NaiveDate> {

    //try to parse from a date like "2023-11-01"
    let date = NaiveDate::parse_from_str(input, "%Y-%m-%d");

    match date {
        Ok(d) => return Some(d), //return formatted date
        _ => {} //try second format
    }

    //try to parse from a date like November 1, 2023
    let date = NaiveDate::parse_from_str(input, "%B %d, %Y");

    match date {
        Ok(d) => return Some(d), //return formatted date
        _ => return None //return input
    }

}
