use serde::{Serialize, Deserialize};
///The decision of the loan application.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CraLoanApplicationDecision {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "OTHER")]
    Other,
}
