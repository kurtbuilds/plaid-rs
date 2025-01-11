use serde::{Serialize, Deserialize};
///RiskProfile is deprecated, use `ruleset` instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskProfile {
    ///The key of the risk profile used for this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    ///The evaluated outcome for this transaction. You can configure a list of outcomes, such as "accept", "review", and "decline" using the Signal dashboard located within the Plaid Dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
}
impl std::fmt::Display for RiskProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
