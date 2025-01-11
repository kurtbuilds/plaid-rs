use serde::{Serialize, Deserialize};
use super::{TransferDiligenceStatus, TransferPlatformRequirement};
///Originator and their status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedOriginator {
    ///The company name of the end customer.
    pub company_name: String,
    ///List of outstanding requirements for scaled platform originators. Only populated when `transfer_diligence_status` is `more_information_required`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outstanding_requirements: Option<Vec<TransferPlatformRequirement>>,
    ///Originator’s diligence status.
    pub transfer_diligence_status: TransferDiligenceStatus,
}
impl std::fmt::Display for DetailedOriginator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
