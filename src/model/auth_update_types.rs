use serde::{Serialize, Deserialize};
///The possible types of auth data that may have changed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthUpdateTypes {
    #[serde(rename = "ACCOUNT_NUMBER")]
    AccountNumber,
    #[serde(rename = "ROUTING_NUMBER")]
    RoutingNumber,
}
