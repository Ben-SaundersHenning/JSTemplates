use sqlite::State;
use std::string::String;
use serde::{Serialize, Deserialize};
use crate::request_builder;

const DB_PATH: &str = if cfg!(windows) {
    "B:\\projects\\JSTG\\JSTG.sqlite3"
} 
else {
    "/home/ben/projects/JSTG/JSTG.sqlite3"
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessor {
    salutation: String,
    first_name: String,
    last_name: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralCompanyListing {
    unique_id: i64,
    common_name: String,
}

pub fn get_all_assessor_info() -> Vec<Assessor> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT Salutation, FirstName, LastName FROM [Assessors];";
    let mut statement = connection.prepare(query).unwrap();

    let mut assessors: Vec<Assessor> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let assessor = Assessor {
            salutation: statement.read::<String, _>("Salutation").unwrap(),
            first_name: statement.read::<String, _>("FirstName").unwrap(),
            last_name: statement.read::<String, _>("LastName").unwrap(),
        };
        assessors.push(assessor);
    }

    assessors

}

pub fn get_referral_company_options() -> Vec<ReferralCompanyListing> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT ReferralCompanyID, CommonName FROM [ReferralCompanies]
                 ORDER BY CommonName ASC;";
    let mut statement = connection.prepare(query).unwrap();

    let mut companies: Vec<ReferralCompanyListing> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let company = ReferralCompanyListing {
            unique_id: statement.read::<i64, _>("ReferralCompanyID").unwrap(),
            common_name: statement.read::<String, _>("CommonName").unwrap(),
        };

        companies.push(company);

    }

    companies

}

pub fn get_referral_company(referral_company: ReferralCompanyListing) -> Option<request_builder::ReferralCompany> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT Name, Address, City, Province, ProvinceAb, PostalCode, Phone, Fax, Email FROM [ReferralCompanies]
                 WHERE ReferralCompanyID = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, referral_company.unique_id)).unwrap();

    while let Ok(State::Row) = statement.next() {
        let company = request_builder::ReferralCompany {
            name: statement.read::<String, _>("Name").unwrap(),
            address: match statement.read::<String, _>("Address") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            city: match statement.read::<String, _>("City") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            province: match statement.read::<String, _>("Province") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            province_ab: match statement.read::<String, _>("ProvinceAb") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            postal_code: match statement.read::<String, _>("PostalCode") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            phone: match statement.read::<String, _>("Phone") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            fax: match statement.read::<String, _>("Fax") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            email: match statement.read::<String, _>("Email") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
        };

        return Some(company);

    }

    None

}

//func to help retrieve absolute paths
//on different machines during development.
#[tauri::command]
pub fn get_path(system: &str, dir: &str) -> String {
    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "
        SELECT Path FROM [Paths]
        WHERE OperatingSystem = :os
        AND Directory = :dir";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((":os", system)).unwrap();
    statement.bind((":dir", dir)).unwrap();

    while let Ok(State::Row) = statement.next() {
        return statement.read::<String, _>("Path").unwrap();
    }

    "NA".to_string()
}
