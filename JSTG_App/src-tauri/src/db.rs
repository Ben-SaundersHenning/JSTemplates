use sqlite::State;
use std::string::String;
use crate::structs::{Assessor, AssessorListing, ReferralCompanyListing, ReferralCompany};

const DB_PATH: &str = if cfg!(windows) {
    "B:\\projects\\JSTG\\JSTG.sqlite3"
} 
else {
    "/home/ben/projects/JSTG/JSTG.sqlite3"
};

pub fn get_assessor_options() -> Vec<AssessorListing> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT RegistrationID, FirstName, LastName FROM [Assessors]
                 ORDER BY FirstName ASC;";
    let mut statement = connection.prepare(query).unwrap();

    let mut assessors: Vec<AssessorListing> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let assessor = AssessorListing {
            registration_id: statement.read::<String, _>("RegistrationID").unwrap(),
            first_name: statement.read::<String, _>("FirstName").unwrap(),
            last_name: statement.read::<String, _>("LastName").unwrap(),
        };
        assessors.push(assessor);
    }

    assessors

}

pub fn get_assessor(assessor: AssessorListing) -> Option<Assessor> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT FirstName, LastName, Salutation, Email, QualificationsParagraph FROM [Assessors]
                 WHERE RegistrationID = ?;";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, assessor.registration_id.as_str())).unwrap();

    while let Ok(State::Row) = statement.next() {
        let assessor = Assessor {
            registration_id: assessor.registration_id,
            first_name: match statement.read::<String, _>("FirstName") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            last_name: match statement.read::<String, _>("LastName") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            salutation: match statement.read::<String, _>("Salutation") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            email: match statement.read::<String, _>("Email") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
            qualifications: match statement.read::<String, _>("QualificationsParagraph") {
                Ok(val) => val,
                _ => "NULL".to_string()
            },
        };

        return Some(assessor);

    }

    None

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

pub fn get_referral_company(referral_company: ReferralCompanyListing) -> Option<ReferralCompany> {

    let connection = sqlite::open(DB_PATH).unwrap();
    let query = "SELECT Name, Address, City, Province, ProvinceAb, PostalCode, Phone, Fax, Email FROM [ReferralCompanies]
                 WHERE ReferralCompanyID = ?;";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, referral_company.unique_id)).unwrap();

    while let Ok(State::Row) = statement.next() {
        let company = ReferralCompany {
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
