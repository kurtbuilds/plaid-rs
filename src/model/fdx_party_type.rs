use serde::{Serialize, Deserialize};
///Identifies the type of a party
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxPartyType {
    #[serde(rename = "DATA_ACCESS_PLATFORM")]
    DataAccessPlatform,
    #[serde(rename = "DATA_PROVIDER")]
    DataProvider,
    #[serde(rename = "DATA_RECIPIENT")]
    DataRecipient,
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "MERCHANT")]
    Merchant,
    #[serde(rename = "VENDOR")]
    Vendor,
}
