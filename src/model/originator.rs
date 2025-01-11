use serde::{Serialize, Deserialize};
use super::TransferDiligenceStatus;
///Originator and their status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Originator {
    ///Originator’s diligence status.
    pub transfer_diligence_status: TransferDiligenceStatus,
}
impl std::fmt::Display for Originator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
