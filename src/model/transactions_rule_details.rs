use serde::{Serialize, Deserialize};
use super::{TransactionsRuleField, TransactionsRuleType};
///A representation of transactions rule details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRuleDetails {
    ///Transaction field for which the rule is defined.
    pub field: TransactionsRuleField,
    ///For TRANSACTION_ID field, provide transaction_id. For NAME field, provide a string pattern.
    pub query: String,
    /**Transaction rule's match type. For TRANSACTION_ID field, EXACT_MATCH is available.
Matches are case sensitive.*/
    #[serde(rename = "type")]
    pub type_: TransactionsRuleType,
}
impl std::fmt::Display for TransactionsRuleDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
