use serde::{Serialize, Deserialize};
///The status of the transfer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BankTransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "reversed")]
    Reversed,
}
