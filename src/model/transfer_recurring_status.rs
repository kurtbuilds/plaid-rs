use serde::{Serialize, Deserialize};
/**The status of the recurring transfer.

`active`: The recurring transfer is currently active.
`cancelled`: The recurring transfer was cancelled by the client or Plaid.
`expired`: The recurring transfer has completed all originations according to its recurring schedule.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferRecurringStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
