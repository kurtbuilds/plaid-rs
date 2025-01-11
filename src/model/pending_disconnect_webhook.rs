use serde::{Serialize, Deserialize};
use super::{PendingDisconnectWebhookReason, WebhookEnvironmentValues};
///Fired when an Item is expected to be disconnected. The webhook will currently be fired 7 days before the existing Item is scheduled for disconnection. This can be resolved by having the user go through Link’s [update mode](http://plaid.com/docs/link/update-mode). Currently, this webhook is fired only for US or Canadian institutions; in the UK or EU, you should continue to listed for the [`PENDING_EXPIRATION`](https://plaid.com/docs/api/items/#pending_expiration) webhook instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDisconnectWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    /**Reason why the item is about to be disconnected.
`INSTITUTION_MIGRATION`: The institution is moving to API or to a different integration. For example, this can occur when an institution moves from a non-OAuth integration to an OAuth integration.
`INSTITUTION_TOKEN_EXPIRATION`: The consent on an Item associated with a US or CA institution is about to expire.*/
    pub reason: PendingDisconnectWebhookReason,
    ///`PENDING_DISCONNECT`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for PendingDisconnectWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
