use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Mrb {
    pub date_of_ocf_18: NaiveDate,
    pub assessor: String,
    pub amount_of_ocf_18: String,
}
