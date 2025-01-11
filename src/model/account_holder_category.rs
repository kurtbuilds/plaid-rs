use serde::{Serialize, Deserialize};
///Indicates the account's categorization as either a personal or a business account. This field is currently in beta; to request access, contact your account manager.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountHolderCategory {
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "personal")]
    Personal,
    #[serde(rename = "unrecognized")]
    Unrecognized,
}
