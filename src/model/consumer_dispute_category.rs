use serde::{Serialize, Deserialize};
///Type of data being disputed by the consumer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConsumerDisputeCategory {
    #[serde(rename = "TRANSACTION")]
    Transaction,
    #[serde(rename = "BALANCE")]
    Balance,
    #[serde(rename = "IDENTITY")]
    Identity,
    #[serde(rename = "OTHER")]
    Other,
}
