use serde::{Serialize, Deserialize};
use super::DashboardUserStatus;
///Account information associated with a team member with access to the Plaid dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUser {
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///A valid email address. Must not have leading or trailing spaces and address must be RFC compliant. For more information, see [RFC 3696](https://datatracker.ietf.org/doc/html/rfc3696).
    pub email_address: String,
    ///ID of the associated user. To retrieve the email address or other details of the person corresponding to this id, use `/dashboard_user/get`.
    pub id: String,
    ///The current status of the user.
    pub status: DashboardUserStatus,
}
impl std::fmt::Display for DashboardUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
