use serde::{Serialize, Deserialize};
///The possible types of identity data that may have changed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IdentityUpdateTypes {
    #[serde(rename = "PHONES")]
    Phones,
    #[serde(rename = "ADDRESSES")]
    Addresses,
    #[serde(rename = "EMAILS")]
    Emails,
    #[serde(rename = "NAMES")]
    Names,
}
