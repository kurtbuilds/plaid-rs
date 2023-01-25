
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityReviewCreateResponse {
    pub audit_trail: WatchlistScreeningAuditTrail,
    pub comment: Option<String>,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub id: String,
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningEntityReviewCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}