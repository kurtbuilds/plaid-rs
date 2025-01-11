use serde::{Serialize, Deserialize};
///The explicit pay basis on the paystub (if present).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditPayStubPayBasisType {
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "HOURLY")]
    Hourly,
    #[serde(rename = "COMMISSION")]
    Commission,
}
