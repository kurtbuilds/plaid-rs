use serde::{Serialize, Deserialize};
///The status of the end customer's security questionnaire.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PartnerEndCustomerQuestionnaireStatus {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "COMPLETE")]
    Complete,
}
