use sqlx::{postgres::PgConnection, Connection};
use std::string::String;
use crate::structs::{Assessor, AssessorListing, ReferralCompanyListing, ReferralCompany, AssessmentType, Path};

const DB_PATH: &str = if cfg!(windows) {
    "B:\\projects\\JSTG\\JSTG.sqlite3"
} 
else {
    "/home/ben/projects/JSTG/JSTG.sqlite3"
};

pub async fn get_assessment_types() -> Result<Vec<AssessmentType>, sqlx::Error> {

    let mut conn = PgConnection::connect("jdbc:postgresql://jsotqln01:5432/jsot").await?;
    let query = "SELECT name, common_name FROM \"assessment_types\";";

    let types = sqlx::query_as::<_, AssessmentType>(query)
        .fetch_all(&mut conn).await?;

    Ok(types)

}

pub async fn get_assessor_options() -> Result<Vec<AssessorListing>, sqlx::Error> {

    let mut conn = PgConnection::connect("jdbc:postgresql://jsotqln01:5432/jsot").await?;
    let query = "SELECT RegistrationID, FirstName, LastName FROM [Assessors]
                 ORDER BY FirstName ASC;";

    let assessors = sqlx::query_as::<_, AssessorListing>(query)
        .fetch_all(&mut conn).await?;

    Ok(assessors)

    // let connection = sqlite::open(DB_PATH).unwrap();
    // let mut statement = connection.prepare(query).unwrap();
    //
    // let mut assessors: Vec<AssessorListing> = Vec::new();
    //
    // while let Ok(State::Row) = statement.next() {
    //     let assessor = AssessorListing {
    //         registration_id: statement.read::<String, _>("RegistrationID").unwrap(),
    //         first_name: statement.read::<String, _>("FirstName").unwrap(),
    //         last_name: statement.read::<String, _>("LastName").unwrap(),
    //     };
    //     assessors.push(assessor);
    // }
    //
    // assessors

}

pub async fn get_assessor(assessor: AssessorListing) -> Result<Option<Assessor>, sqlx::Error> {

    let mut conn = PgConnection::connect("jdbc:postgresql://jsotqln01:5432/jsot").await?;
    let query = "SELECT FirstName, LastName, Title, Email, QualificationsParagraph FROM [Assessors]
                 WHERE RegistrationID = ?;";

    let assessor = sqlx::query_as::<_, Assessor>(query)
        .bind(assessor.registration_id)
        .fetch_optional(&mut conn).await?;

    Ok(assessor)

    // let connection = sqlite::open(DB_PATH).unwrap();
    // let query = "SELECT FirstName, LastName, Title, Email, QualificationsParagraph FROM [Assessors]
    //              WHERE RegistrationID = ?;";
    // let mut statement = connection.prepare(query).unwrap();
    // statement.bind((1, assessor.registration_id.as_str())).unwrap();
    //
    // while let Ok(State::Row) = statement.next() {
    //     let assessor = Assessor {
    //         registration_id: assessor.registration_id,
    //         first_name: match statement.read::<String, _>("FirstName") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //         last_name: match statement.read::<String, _>("LastName") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //         title: match statement.read::<String, _>("Title") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //         email: match statement.read::<String, _>("Email") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //         qualifications: match statement.read::<String, _>("QualificationsParagraph") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //     };
    //
    //     return Some(assessor);
    //
    // }
    //
    // None

}

pub async fn get_referral_company_options() -> Result<Vec<ReferralCompanyListing>, sqlx::Error> {

    let mut conn = PgConnection::connect("jdbc:postgresql://jsotqln01:5432/jsot").await?;
    let query = "SELECT ReferralCompanyID, CommonName FROM [ReferralCompanies]
                 ORDER BY CommonName ASC;";

    let companies = sqlx::query_as::<_, ReferralCompanyListing>(query)
        .fetch_all(&mut conn).await?;

    Ok(companies)

    // let connection = sqlite::open(DB_PATH).unwrap();
    // let mut statement = connection.prepare(query).unwrap();
    //
    // let mut companies: Vec<ReferralCompanyListing> = Vec::new();
    //
    // while let Ok(State::Row) = statement.next() {
    //     let company = ReferralCompanyListing {
    //         unique_id: statement.read::<i64, _>("ReferralCompanyID").unwrap(),
    //         common_name: statement.read::<String, _>("CommonName").unwrap(),
    //     };
    //
    //     companies.push(company);
    //
    // }
    //
    // companies

}

pub async fn get_referral_company(referral_company: ReferralCompanyListing) -> Result<Option<ReferralCompany>, sqlx::Error> {

    let mut conn = PgConnection::connect("jdbc:postgresql://jsotqln01:5432/jsot").await?;
    let query = "SELECT Name, Address, City, Province, ProvinceAb, PostalCode, Phone, Fax, Email FROM [ReferralCompanies]
                 WHERE ReferralCompanyID = ?;";

    let company = sqlx::query_as::<_, ReferralCompany>(query)
        .bind(referral_company.unique_id)
        .fetch_optional(&mut conn).await?;

    Ok(company)

    // let connection = sqlite::open(DB_PATH).unwrap();
    // let query = "SELECT Name, Address, City, Province, ProvinceAb, PostalCode, Phone, Fax, Email FROM [ReferralCompanies]
    //              WHERE ReferralCompanyID = ?;";
    // let mut statement = connection.prepare(query).unwrap();
    // statement.bind((1, referral_company.unique_id)).unwrap();
    //
    // while let Ok(State::Row) = statement.next() {
    //     let company = ReferralCompany {
    //         unique_id: referral_company.unique_id,
    //         name: statement.read::<String, _>("Name").unwrap(),
    //         common_name: referral_company.common_name,
    //         address: Address {
    //             address: match statement.read::<String, _>("Address") {
    //                 Ok(val) => val,
    //                 _ => "NULL".to_string()
    //             },
    //             city: match statement.read::<String, _>("City") {
    //                 Ok(val) => val,
    //                 _ => "NULL".to_string()
    //             },
    //             province: match statement.read::<String, _>("Province") {
    //                 Ok(val) => val,
    //                 _ => "NULL".to_string()
    //             },
    //             province_ab: match statement.read::<String, _>("ProvinceAb") {
    //                 Ok(val) => val,
    //                 _ => "NULL".to_string()
    //             },
    //             postal_code: match statement.read::<String, _>("PostalCode") {
    //                 Ok(val) => val,
    //                 _ => "NULL".to_string()
    //             },
    //             country: "Canada".to_string(),
    //             address_long: "".to_string() //needs to be built
    //         },
    //         phone: match statement.read::<String, _>("Phone") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //         fax: match statement.read::<String, _>("Fax") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //         email: match statement.read::<String, _>("Email") {
    //             Ok(val) => val,
    //             _ => "NULL".to_string()
    //         },
    //     };
    //
    //     return Some(company);
    //
    // }
    //
    // None

}

//func to help retrieve absolute paths
//on different machines during development.
pub async fn get_path(system: &str, dir: &str) -> Result<String, sqlx::Error> {

    let mut conn = PgConnection::connect("jdbc:postgresql://jsotqln01:5432/jsot").await?;
    let query = "
        SELECT Path FROM [Paths]
        WHERE OperatingSystem = ?
        AND Directory = ?";

    let path = sqlx::query_as::<_, Path>(query)
        .bind(system)
        .bind(dir)
        .fetch_one(&mut conn).await?;


    Ok(path.path)

    // let connection = sqlite::open(DB_PATH).unwrap();
    // let query = "
    //     SELECT Path FROM [Paths]
    //     WHERE OperatingSystem = :os
    //     AND Directory = :dir";
    // let mut statement = connection.prepare(query).unwrap();
    // statement.bind((":os", system)).unwrap();
    // statement.bind((":dir", dir)).unwrap();
    //
    // while let Ok(State::Row) = statement.next() {
    //     return statement.read::<String, _>("Path").unwrap();
    // }
    //
    // "NA".to_string()
}
