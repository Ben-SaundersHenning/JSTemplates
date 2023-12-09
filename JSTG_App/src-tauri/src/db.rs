use sqlx::{postgres::PgConnection, Connection, Row};
use std::string::String;
use std::error::Error;
use crate::structs::{Assessor, AssessorListing, ReferralCompanyListing, ReferralCompany, Document, Path, Address};
use std::env;
// JSTG_DEV_DB

const DB_CONN_STR: &str = "JSTG_DB_POSTGRESQL";

pub async fn get_document_options() -> Result<Vec<Document>, Box<dyn Error>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;
    let query = "SELECT id, common_name, file FROM \"documents\";";

    let documents = sqlx::query_as::<_, Document>(query)
        .fetch_all(&mut conn).await?;

    Ok(documents)

}

pub async fn get_assessor_options() -> Result<Vec<AssessorListing>, Box<dyn Error>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;
    let query = "SELECT registration_id, first_name, last_name FROM \"assessors\"
                 ORDER BY first_name ASC;";

    let assessors = sqlx::query_as::<_, AssessorListing>(query)
        .fetch_all(&mut conn).await?;

    Ok(assessors)

}

pub async fn get_assessor(assessor: AssessorListing) -> Option<Assessor> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await.unwrap();
    let query = "SELECT first_name, last_name, title, email, qualification_paragrpah FROM \"assessors\"
                WHERE registration_id = ?;";

    // match sqlx::query_as::<_, Assessor>(query)
    //     .bind(assessor.registration_id)
    //     .fetch_optional(&mut conn).await {
    //     Ok(val) => return val,
    //     Err(e) => {println!("ERROR: {0}", e.to_string()); return None;}
    //     };

    match sqlx::query_as!(Assessor, "
                          SELECT registration_id, first_name, last_name, title, email, qualifications_paragraph
                          FROM \"assessors\"
                          WHERE registration_id = $1", assessor.registration_id)
        .fetch_optional(&mut conn).await {
        Ok(val) => return val,
        Err(e) => {println!("ERROR: {0}", e.to_string()); return None;}
        };

    // Ok(assessor)

}

pub async fn get_referral_company_options() -> Result<Vec<ReferralCompanyListing>, Box<dyn Error>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;
    let query = "SELECT id, common_name FROM \"referral_companies\"
                 ORDER BY common_name ASC;";

    let companies = sqlx::query_as::<_, ReferralCompanyListing>(query)
        .fetch_all(&mut conn).await?;

    Ok(companies)

}

// pub async fn get_referral_company(referral_company: ReferralCompanyListing) -> Result<ReferralCompany, Box<dyn Error>> {
pub async fn get_referral_company(referral_company: ReferralCompanyListing) -> Option<ReferralCompany> {

    println!("In method");

    println!("ID = {0}", referral_company.id);

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await.unwrap();
    let query = "SELECT name, common_name, phone, fax, email, address, city, province, province_abbreviated, postal_code
                FROM referral_companies
                WHERE id = ?";

    let row = match sqlx::query!("SELECT name, common_name, phone, fax, email,
                           address, city, province, province_abbreviated, postal_code
                           FROM \"referral_companies\"
                           WHERE \"id\" = $1;", referral_company.id)
        .fetch_optional(&mut conn).await {
        Ok(val) => val,
        Err(e) => {println!("{0}", e.to_string()); panic!("{0}", e.to_string());}
        };

    println!("after the query");
    match row {
        Some(data) => {
            let company = ReferralCompany {
                name: data.name,
                common_name: data.common_name,
                phone: data.phone,
                fax: data.fax,
                email: data.email,
                address: Address {
                    address: data.address,
                    city: data.city,
                    province: data.province,
                    province_abbreviated: data.province_abbreviated,
                    postal_code: data.postal_code,
                    country: "Canada".to_string(),
                    address_long: "".to_string()
                }
            };

            println!("Company has been created");
            println!("Comapny name: {0}", company.name);
            return Some(company);
        },
        None => {println!("Row had nothing");}
    };

    // let row = match sqlx::query(query)
    //     .bind(referral_company.id)
    //     .fetch_optional(&mut conn).await {
    //     Ok(val) => val,
    //     Err(e) => {println!("{0}", e.to_string()); panic!("{0}", e.to_string());}
    //     };

    // println!("have row");
    //
    // match row {
    //     Some(data) => {
    //         return Ok(ReferralCompany {
    //             name: data.try_get("name")?,
    //             common_name: data.try_get("common_name")?,
    //             phone: data.try_get("phone")?,
    //             fax: data.try_get("fax")?,
    //             email: data.try_get("email")?,
    //             address: Address {
    //                 address: data.try_get("address")?,
    //                 city: data.try_get("city")?,
    //                 province: data.try_get("province")?,
    //                 province_abbreviated: data.try_get("province_abbreviated")?,
    //                 postal_code: data.try_get("postal_code")?,
    //                 country: "Canada".to_string(),
    //                 address_long: "".to_string()
    //             }
    //         });
    //     },
    //     None => {}
    // };

    println!("returning err");

    // Err("error")?
    None

}

//func to help retrieve absolute paths
//on different machines during development.
pub async fn get_path(system: &str, dir: &str) -> Result<String, Box<dyn Error>> {

    let mut conn = PgConnection::connect(&env::var(DB_CONN_STR).unwrap()).await?;
    let query = "
        SELECT path FROM \"paths\"
        WHERE operating_system = ?
        AND directory = ?";

    let path = sqlx::query_as::<_, Path>(query)
        .bind(system)
        .bind(dir)
        .fetch_one(&mut conn).await?;

    Ok(path.path)

}
