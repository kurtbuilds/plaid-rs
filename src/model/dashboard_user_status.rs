use serde::{Serialize, Deserialize};
///The current status of the user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DashboardUserStatus {
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deactivated")]
    Deactivated,
}
