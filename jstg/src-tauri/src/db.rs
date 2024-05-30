use sqlx::{postgres::PgConnection, Connection};
use std::env;
use serde::Serialize;
use crate::Error;

const DB_CONN_STR: &str = "JSTG_DB_POSTGRESQL";

#[derive(Serialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other
}

#[derive(Serialize, sqlx::FromRow)]
pub struct JsonListing {
    pub listing_details: sqlx::types::JsonValue,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Assessor {
    pub registration_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub gender: Gender,
    pub qualifications_paragraph: String
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ReferralCompany {
    pub name: String,
    pub common_name: String,
    pub phone: String,
    pub fax: String,
    pub email: String,
    #[sqlx(flatten)]
    pub address: Address,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Address {
    pub address: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,
    pub country: String,
}

// Retrieves the set of assessors
// (name, id)
#[tauri::command]
pub async fn get_assessor_options() -> Result<JsonListing, Error> {

    let mut conn_str: String = String::new();

    // dev environment
    if cfg!(dev) {
        conn_str.push_str("postgres://jstg:password@localhost:5432/jsot");
    } else {
        conn_str.push_str(&env::var(DB_CONN_STR).unwrap());
    }

    let mut conn = PgConnection::connect(&conn_str).await?;

    // Fetches a JSON array of JSON objects,
    // where each object represents one company.
    let query = "SELECT json_agg(json_build_object(
                    'name', assessors.first_name || ' ' || assessors.last_name,
                    'id', trim(assessors.registration_id)
                    )) as listing_details
                 FROM \"assessors\";";

    let assessors = sqlx::query_as::<_, JsonListing>(query)
        .fetch_one(&mut conn).await?;

    conn.close().await?;

    Ok(assessors)

}

// Retrieives an assessor from the database based on
// a given unique ID.
pub async fn get_assessor(registration_id: &str) -> Result<Option<Assessor>, Error> {

    let mut conn_str: String = String::new();

    // dev environment
    if cfg!(dev) {
        conn_str.push_str("postgres://jstg:password@localhost:5432/jsot");
    } else {
        conn_str.push_str(&env::var(DB_CONN_STR).unwrap());
    }

    let mut conn = PgConnection::connect(&conn_str).await?;

    let query = "SELECT registration_id,
                        first_name,
                        last_name,
                        gender,
                        email,
                        qualifications_paragraph
                 FROM \"assessors\"
                 WHERE registration_id = $1";

    let assessor = sqlx::query_as::<_, Assessor>(query)
        .bind(registration_id)
        .fetch_optional(&mut conn).await?;

    conn.close().await?;

    Ok(assessor)

}

// Retrieves the set of companies
// (name, id)
#[tauri::command]
pub async fn get_referral_company_options() -> Result<JsonListing, Error> {

    let mut conn_str: String = String::new();

    // dev environment
    if cfg!(dev) {
        conn_str.push_str("postgres://jstg:password@localhost:5432/jsot");
    } else {
        conn_str.push_str(&env::var(DB_CONN_STR).unwrap());
    }

    let mut conn = PgConnection::connect(&conn_str).await?;

    // Fetches a JSON array of JSON objects,
    // where each object represents one company.
    let query = "SELECT json_agg(json_build_object(
                    'name', rc.common_name,
                    'id', rc.id
                    )) as listing_details
                 FROM \"referral_companies\" rc;";

    let companies = sqlx::query_as::<_, JsonListing>(query)
        .fetch_one(&mut conn).await?;

    conn.close().await?;

    Ok(companies)

}

// Retrieives a company from the database based on
// a given unique ID.
pub async fn get_referral_company(referral_company_id: i32) -> Result<Option<ReferralCompany>, Error> {

    let mut conn_str: String = String::new();

    // dev environment
    if cfg!(dev) {
        conn_str.push_str("postgres://jstg:password@localhost:5432/jsot");
    } else {
        conn_str.push_str(&env::var(DB_CONN_STR).unwrap());
    }

    let mut conn = PgConnection::connect(&conn_str).await?;

    let query = "SELECT name,
                        common_name,
                        phone,
                        fax,
                        email,
                        address,
                        postal_code,
                        city,
                        province,
                        country
                 FROM \"referral_companies\"
                 WHERE \"id\" = $1;";

    let company = sqlx::query_as::<_, ReferralCompany>(query)
        .bind(referral_company_id)
        .fetch_optional(&mut conn).await?;

    conn.close().await?;

    Ok(company)

}
