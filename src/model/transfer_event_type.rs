use serde::{Serialize, Deserialize};
/**The type of event that this transfer represents. Event types with prefix `sweep` represents events for Plaid Ledger sweeps.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`settled`: Credits are available to be withdrawn or debits have been deducted from the Plaid linked account.

`funds_available`: Funds from the transfer have been released from hold and applied to the ledger's available balance. (Only applicable to ACH debits.)

`returned`: A posted transfer was returned.

`swept`: The transfer was swept to / from the sweep account.

`swept_settled`: Credits are available to be withdrawn or debits have been deducted from the customer’s business checking account.

`return_swept`: Due to the transfer being returned, funds were pulled from or pushed back to the sweep account.

`sweep.pending`: A new ledger sweep was created; it is in the pending state.

`sweep.posted`: The ledger sweep has been successfully submitted to the payment network.

`sweep.settled`: The transaction has settled in the funding account. This means that funds withdrawn from Plaid Ledger balance have reached the funding account, or funds to be deposited into the Plaid Ledger Balance have been pulled, and the hold period has begun.

`sweep.returned`: A posted ledger sweep was returned.

`sweep.failed`: The ledger sweep failed, no funds were moved.

`refund.pending`: A new refund was created; it is in the pending state.

`refund.cancelled`: The refund was cancelled.

`refund.failed`: The refund failed, no funds were moved.

`refund.posted`: The refund has been successfully submitted to the payment network.

`refund.settled`: The refund transaction has settled in the Plaid linked account.

`refund.returned`: A posted refund was returned.

`refund.swept`: The refund was swept from the sweep account.

`refund.return_swept`: Due to the refund being returned, funds were pushed back to the sweep account.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "funds_available")]
    FundsAvailable,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "swept_settled")]
    SweptSettled,
    #[serde(rename = "return_swept")]
    ReturnSwept,
    #[serde(rename = "sweep.pending")]
    SweepPending,
    #[serde(rename = "sweep.posted")]
    SweepPosted,
    #[serde(rename = "sweep.settled")]
    SweepSettled,
    #[serde(rename = "sweep.returned")]
    SweepReturned,
    #[serde(rename = "sweep.failed")]
    SweepFailed,
    #[serde(rename = "refund.pending")]
    RefundPending,
    #[serde(rename = "refund.cancelled")]
    RefundCancelled,
    #[serde(rename = "refund.failed")]
    RefundFailed,
    #[serde(rename = "refund.posted")]
    RefundPosted,
    #[serde(rename = "refund.settled")]
    RefundSettled,
    #[serde(rename = "refund.returned")]
    RefundReturned,
    #[serde(rename = "refund.swept")]
    RefundSwept,
    #[serde(rename = "refund.return_swept")]
    RefundReturnSwept,
}
