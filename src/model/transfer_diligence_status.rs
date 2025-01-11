use serde::{Serialize, Deserialize};
///Originator’s diligence status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferDiligenceStatus {
    #[serde(rename = "not_submitted")]
    NotSubmitted,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "more_information_required")]
    MoreInformationRequired,
}
