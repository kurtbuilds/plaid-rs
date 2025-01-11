use serde::{Serialize, Deserialize};
///Enum representing the entity authenticating the user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ItemCreateAuthentication {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "DATA_PARTNER")]
    DataPartner,
    #[serde(rename = "PLAID")]
    Plaid,
}
