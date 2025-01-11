use serde::{Serialize, Deserialize};
/**Transaction rule's match type. For TRANSACTION_ID field, EXACT_MATCH is available.
Matches are case sensitive.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionsRuleType {
    #[serde(rename = "EXACT_MATCH")]
    ExactMatch,
    #[serde(rename = "SUBSTRING_MATCH")]
    SubstringMatch,
}
