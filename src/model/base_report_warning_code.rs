use serde::{Serialize, Deserialize};
///The warning code identifies a specific kind of warning. `OWNERS_UNAVAILABLE` indicates that account-owner information is not available. `TRANSACTIONS_UNAVAILABLE` indicates that transactions information associated with Credit and Depository accounts are unavailable. `USER_FRAUD_ALERT` indicates that the User has placed a fraud alert on their Plaid Check consumer report due to suspected fraud.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BaseReportWarningCode {
    #[serde(rename = "OWNERS_UNAVAILABLE")]
    OwnersUnavailable,
    #[serde(rename = "TRANSACTIONS_UNAVAILABLE")]
    TransactionsUnavailable,
    #[serde(rename = "USER_FRAUD_ALERT")]
    UserFraudAlert,
}
