use serde::{Serialize, Deserialize};
///The pay type - `GROSS`, `NET`, or `UNKNOWN` for a specified income source
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatedIncomeSourcePayType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "GROSS")]
    Gross,
    #[serde(rename = "NET")]
    Net,
}
