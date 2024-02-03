use std::error::Error;
use std::env;
use crate::structs::{Document, AssessorListing, Assessor, ReferralCompanyListing, ReferralCompany, Path, Assessment};
use sqlx::{PgConnection, Connection};
use serde_json::Value;
use log::info;

const DB_CONN_STR: &str = "JSTG_DB_POSTGRESQL";

// async fn check_postgresql_connetion() -> bool {
//
//     match &env::var(DB_CONN_STR) {
//         Ok(val) => {
//
//             let conn = PgConnection::connect(val).await;
//
//             match conn {
//                 Ok(pg_conn) => { pg_conn.close(); return true; },
//                 Err(..) => { return false; }
//             };
//
//         },
//         Err(..) => { return false; }
//     };
//
// }

// // Saves a generated request (JSON object) into the database. Values
// // get mapped to corresponding columns.
// pub async fn save_request_to_database(request: Option<Assessment<Value>>) {
//
//     // Mutliple requests can exist for one client.
//     // Each request saved will be given a UID.
//     // In the application, requests will be searched
//     // by name. OTs should also appear.
//
// }
//
// // Retrieves a request from the database based on ID.
// // The request may be used to fill in data to the form.
// pub async fn get_request_from_database() -> Result<Option<Assessment<Value>>, Box<dyn Error + Send + Sync>> {
//
// }
//
// // Retrieves requests from the database 
// // that match some search.
// pub async fn get_requests_from_database() -> Result<Option<Assessment<Value>>, Box<dyn Error + Send + Sync>> {
//
// }

pub async fn get_document_options() -> Result<Vec<Document>, Box<dyn Error + Send + Sync>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;

    let query = "SELECT id,
                        common_name,
                        file
                FROM \"documents\"
                ORDER BY common_name ASC;";

    let documents = sqlx::query_as::<_, Document>(query)
        .fetch_all(&mut conn).await?;

    conn.close().await?;

    Ok(documents)

}

pub async fn get_assessor_options() -> Result<Vec<AssessorListing>, Box<dyn Error + Send + Sync>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;

    let query = "SELECT registration_id,
                        first_name,
                        last_name
                 FROM \"assessors\"
                 ORDER BY first_name ASC;";

    let assessors = sqlx::query_as::<_, AssessorListing>(query)
        .fetch_all(&mut conn).await?;

    conn.close().await?;

    Ok(assessors)

}

pub async fn get_assessor(assessor: AssessorListing) -> Result<Option<Assessor>, Box<dyn Error + Send + Sync>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;

    let query = "SELECT registration_id,
                        title,
                        first_name,
                        last_name,
                        email,
                        qualifications_paragraph
                 FROM \"assessors\"
                 WHERE registration_id = $1";

    let assessor = sqlx::query_as::<_, Assessor>(query)
        .bind(assessor.registration_id)
        .fetch_optional(&mut conn).await?;

    conn.close().await?;

    Ok(assessor)

}

pub async fn get_referral_company_options() -> Result<Vec<ReferralCompanyListing>, Box<dyn Error + Send + Sync>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;

    let query = "SELECT id,
                        common_name
                 FROM \"referral_companies\"
                 ORDER BY common_name ASC;";

    let companies = sqlx::query_as::<_, ReferralCompanyListing>(query)
        .fetch_all(&mut conn).await?;

    conn.close().await?;

    Ok(companies)

}

pub async fn get_referral_company(referral_company: ReferralCompanyListing) -> Result<Option<ReferralCompany>, Box<dyn Error + Send + Sync>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;
    let query = "SELECT name,
                        common_name,
                        phone,
                        fax,
                        email,
                        address,
                        postal_code,
                        city,
                        province,
                        province_abbreviated,
                        country,
                        '' AS address_long
                 FROM \"referral_companies\"
                 WHERE \"id\" = $1;";

    let company = sqlx::query_as::<_, ReferralCompany>(query)
        .bind(referral_company.id)
        .fetch_optional(&mut conn).await?;

    conn.close().await?;

    Ok(company)

}

//func to help retrieve absolute paths
//on different machines during development.
pub async fn get_path(system: &str, dir: &str) -> Result<String, Box<dyn Error + Send + Sync>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;
    let query = "
        SELECT path FROM \"paths_development\"
        WHERE operating_system = $1
        AND directory = $2;";

    let path = sqlx::query_as::<_, Path>(query)
        .bind(system)
        .bind(dir)
        .fetch_one(&mut conn).await?;

    conn.close().await?;

    Ok(path.path)

}
