use serde::{Serialize, Deserialize};
///SMTP-MX check to confirm the email address exists if known.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckEmailIsDeliverableStatus {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
