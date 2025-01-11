use serde::{Serialize, Deserialize};
///The status of the addendum to the Plaid MSA ("flowdown") for the end customer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PartnerEndCustomerFlowdownStatus {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "IN_REVIEW")]
    InReview,
    #[serde(rename = "NEGOTIATION")]
    Negotiation,
    #[serde(rename = "COMPLETE")]
    Complete,
}
