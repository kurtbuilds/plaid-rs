use serde::{Serialize, Deserialize};
///Details about the transaction result after evaluated by the requested Ruleset. If a `ruleset_key` is not provided, this field will be omitted. This feature is currently in closed beta; to request access, contact your account manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ruleset {
    ///The evaluated outcome for this transaction. You can configure a list of outcomes, such as "accept", "review", and "decline" using the Signal dashboard located within the Plaid Dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    ///The key of the Ruleset used for this transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruleset_key: Option<String>,
}
impl std::fmt::Display for Ruleset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
