
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityDefaultUpdateWebhook {
    pub account_ids_with_updated_identity: AccountIdsWithUpdatedIdentity,
    pub environment: String,
    pub error: Option<PlaidError>,
    pub item_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for IdentityDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}