use serde::{Serialize, Deserialize};
///The types of source employment data that users should be able to share
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EmploymentSourceType {
    #[serde(rename = "bank")]
    Bank,
    #[serde(rename = "payroll")]
    Payroll,
}
