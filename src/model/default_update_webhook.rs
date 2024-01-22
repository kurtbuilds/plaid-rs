use serde::{Serialize, Deserialize};
use super::PlaidError;
/**Fired when new transaction data is available for an Item. Plaid will typically check for new transaction data several times a day.

This webhook is intended for use with `/transactions/get`; if you are using the newer `/transactions/sync` endpoint, this webhook will still be fired to maintain backwards compatibility, but it is recommended to listen for and respond to the `SYNC_UPDATES_AVAILABLE` webhook instead.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item the webhook relates to.
    pub item_id: String,
    ///The number of new transactions detected since the last time this webhook was fired.
    pub new_transactions: f64,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///`TRANSACTIONS`
    pub webhook_type: String,
}
impl std::fmt::Display for DefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}