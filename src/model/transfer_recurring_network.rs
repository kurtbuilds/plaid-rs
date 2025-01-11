use serde::{Serialize, Deserialize};
///Networks eligible for recurring transfers.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferRecurringNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "rtp")]
    Rtp,
}
