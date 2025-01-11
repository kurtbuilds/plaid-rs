use serde::{Serialize, Deserialize};
/**The error code of a failed transaction. Error codes include:
`EXTERNAL_SYSTEM`: The transaction was declined by an external system.
`EXPIRED`: The transaction request has expired.
`CANCELLED`: The transaction request was rescinded.
`INVALID`: The transaction did not meet certain criteria, such as an inactive account or no valid counterparty, etc.
`UNKNOWN`: The transaction was unsuccessful, but the exact cause is unknown.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WalletTransactionFailureReason {
    #[serde(rename = "EXTERNAL_SYSTEM")]
    ExternalSystem,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
