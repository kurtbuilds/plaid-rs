use serde::{Serialize, Deserialize};
///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
}
