use serde::{Serialize, Deserialize};
///The types of source income data that users should be able to share
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncomeVerificationSourceType {
    #[serde(rename = "bank")]
    Bank,
    #[serde(rename = "payroll")]
    Payroll,
}
