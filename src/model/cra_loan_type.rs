use serde::{Serialize, Deserialize};
///The type of loan the user applied for.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CraLoanType {
    #[serde(rename = "PERSONAL")]
    Personal,
    #[serde(rename = "CREDIT_CARD")]
    CreditCard,
    #[serde(rename = "BUSINESS")]
    Business,
    #[serde(rename = "MORTGAGE")]
    Mortgage,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "PAYDAY")]
    Payday,
    #[serde(rename = "STUDENT")]
    Student,
    #[serde(rename = "HOME_EQUITY")]
    HomeEquity,
    #[serde(rename = "OTHER")]
    Other,
}
