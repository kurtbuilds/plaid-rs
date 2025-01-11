use serde::{Serialize, Deserialize};
///The status of the loan.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CraLoanStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "BOOKED")]
    Booked,
    #[serde(rename = "CURRENT")]
    Current,
    #[serde(rename = "DELINQUENT")]
    Delinquent,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "CHARGED_OFF")]
    ChargedOff,
    #[serde(rename = "TRANSFERRED")]
    Transferred,
    #[serde(rename = "PAID_OFF")]
    PaidOff,
    #[serde(rename = "OTHER")]
    Other,
}
