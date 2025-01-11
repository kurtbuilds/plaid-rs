use serde::{Serialize, Deserialize};
///Enum representing the overall network status of the user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProfileNetworkStatusGetNetworkStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "RETURNING_USER")]
    ReturningUser,
}
