use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when a user successfully adds a Plaid Item during a Link session when using Hosted Link or Multi-Item Link sessions. Contains the public token for the Item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAddResultWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///The identifier for the Link session.
    pub link_session_id: String,
    ///The link token used to create the Link session.
    pub link_token: String,
    ///The public token corresponding to the item that was added.
    pub public_token: String,
    ///`ITEM_ADD_RESULT`
    pub webhook_code: String,
    ///`LINK`
    pub webhook_type: String,
}
impl std::fmt::Display for ItemAddResultWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
