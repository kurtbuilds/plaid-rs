use serde::{Serialize, Deserialize};
use super::{
    LiabilitiesAccountIdsWithUpdatedLiabilities, PlaidError, WebhookEnvironmentValues,
};
///The webhook of type `LIABILITIES` and code `DEFAULT_UPDATE` will be fired when new or updated liabilities have been detected on a liabilities item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilitiesDefaultUpdateWebhook {
    ///An array of `account_id`'s for accounts that contain new liabilities.'
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account_ids_with_new_liabilities: Vec<String>,
    /**An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["past_amount_due"] }`*/
    pub account_ids_with_updated_liabilities: LiabilitiesAccountIdsWithUpdatedLiabilities,
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///`LIABILITIES`
    pub webhook_type: String,
}
impl std::fmt::Display for LiabilitiesDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
