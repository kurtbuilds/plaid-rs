use serde::{Serialize, Deserialize};
///Asset Transaction Type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetTransactionType {
    Credit,
    Debit,
}
