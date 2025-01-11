use serde::{Serialize, Deserialize};
///The pay frequency of a specified income source
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatedIncomeSourceFrequency {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
}
