use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mrb {
    pub date_of_ocf_18: NaiveDate,
    pub assessor: String,
    pub ocf_18_amount: String,
}
