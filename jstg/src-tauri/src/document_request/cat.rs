use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cat {
    pub date_of_ocf_19: NaiveDate,
    pub assessor: String,
}
