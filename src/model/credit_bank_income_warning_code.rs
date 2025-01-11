use serde::{Serialize, Deserialize};
/**The warning code identifies a specific kind of warning.
`IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item
`TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item
`ITEM_UNAPPROVED`: User exited flow before giving permission to share data for the Item
`REPORT_DELETED`: Report deleted due to customer or consumer request
`DATA_UNAVAILABLE`: No relevant data was found for the Item*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditBankIncomeWarningCode {
    #[serde(rename = "IDENTITY_UNAVAILABLE")]
    IdentityUnavailable,
    #[serde(rename = "TRANSACTIONS_UNAVAILABLE")]
    TransactionsUnavailable,
    #[serde(rename = "ITEM_UNAPPROVED")]
    ItemUnapproved,
    #[serde(rename = "REPORT_DELETED")]
    ReportDeleted,
    #[serde(rename = "DATA_UNAVAILABLE")]
    DataUnavailable,
}
