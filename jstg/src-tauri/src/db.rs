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

#[tauri::command]
pub async fn get_assessor(registration_id: &str) -> Result<Assessor, Error> {

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
        .bind(registration_id)
        .fetch_one(&mut conn).await?;

    conn.close().await?;

    Ok(assessor)

}

#[tauri::command]
pub async fn get_referral_company(referral_company_id: i32) -> Result<ReferralCompany, Error> {

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
        .bind(referral_company_id)
        .fetch_one(&mut conn).await?;
        // .fetch_optional(&mut conn).await?;

    conn.close().await?;

    Ok(company)

}
