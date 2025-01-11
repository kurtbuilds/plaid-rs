use serde::{Serialize, Deserialize};
/**The status of the transfer.

`pending`: A new transfer was created; it is in the pending state.
`posted`: The transfer has been successfully submitted to the payment network.
`settled`: Credits are available to be withdrawn or debits have been deducted from the Plaid linked account.
`funds_available`: Funds from the transfer have been released from hold and applied to the ledger's available balance. (Only applicable to ACH debits.)
`cancelled`: The transfer was cancelled by the client.
`failed`: The transfer failed, no funds were moved.
`returned`: A posted transfer was returned.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "funds_available")]
    FundsAvailable,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "returned")]
    Returned,
}
