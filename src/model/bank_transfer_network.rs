use serde::{Serialize, Deserialize};
///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BankTransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "wire")]
    Wire,
}
