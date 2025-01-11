use serde::{Serialize, Deserialize};
///The entity that initiated collection of consent.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConsentEventInitiator {
    #[serde(rename = "PLAID")]
    Plaid,
    #[serde(rename = "DATA_PROVIDER")]
    DataProvider,
    #[serde(rename = "CUSTOMER")]
    Customer,
    #[serde(rename = "END_USER")]
    EndUser,
}
