use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
/**This webhook is only sent to [Plaid processor partners](https://plaid.com/docs/auth/partnerships/).

Fired when an Item's initial transaction pull is completed. Once this webhook has been fired, transaction data for the most recent 30 days can be fetched for the Item. This webhook will also be fired if account selections for the Item are updated, with `new_transactions` set to the number of net new transactions pulled after the account selection update.

This webhook is intended for use with `/processor/transactions/get`; if you are using the newer `/processor/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorInitialUpdateWebhook {
    ///The ID of the account.
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///The error code associated with the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    ///The number of new transactions available.
    pub new_transactions: f64,
    ///`INITIAL_UPDATE`
    pub webhook_code: String,
    ///`TRANSACTIONS`
    pub webhook_type: String,
}
impl std::fmt::Display for ProcessorInitialUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
