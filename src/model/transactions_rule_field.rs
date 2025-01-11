use serde::{Serialize, Deserialize};
///Transaction field for which the rule is defined.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionsRuleField {
    #[serde(rename = "TRANSACTION_ID")]
    TransactionId,
    #[serde(rename = "NAME")]
    Name,
}
