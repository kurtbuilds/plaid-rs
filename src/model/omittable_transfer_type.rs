use serde::{Serialize, Deserialize};
///The type of transfer. Valid values are `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account. This field is omitted for Plaid Ledger Sweep events.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OmittableTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
}
