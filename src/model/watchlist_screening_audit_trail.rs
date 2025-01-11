use serde::{Serialize, Deserialize};
use super::Source;
///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningAuditTrail {
    ///ID of the associated user. To retrieve the email address or other details of the person corresponding to this id, use `/dashboard_user/get`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dashboard_user_id: Option<String>,
    ///A type indicating whether a dashboard user, an API-based user, or Plaid last touched this object.
    pub source: Source,
    ///An ISO8601 formatted timestamp.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for WatchlistScreeningAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
