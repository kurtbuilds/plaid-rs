use serde::{Serialize, Deserialize};
use super::{AccountIdsWithUpdatedAuth, PlaidError, WebhookEnvironmentValues};
///Plaid will trigger a `DEFAULT_UPDATE` webhook for Items that undergo a change in Auth data. This is generally caused by data partners notifying Plaid of a change in their account numbering system or to their routing numbers. To avoid returned transactions, customers that receive a `DEFAULT_UPDATE` webhook with the `account_ids_with_updated_auth` object populated should immediately discontinue all usages of existing Auth data for those accounts and call `/auth/get` or `/processor/auth/get` to obtain updated account and routing numbers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefaultUpdateWebhook {
    ///An array of `account_id`'s for accounts that contain new auth.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account_ids_with_new_auth: Vec<String>,
    /**An object with keys of `account_id`'s that are mapped to their respective auth attributes that changed. `ACCOUNT_NUMBER` and `ROUTING_NUMBER` are the two potential values that can be flagged as updated.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["ACCOUNT_NUMBER"] }`*/
    pub account_ids_with_updated_auth: AccountIdsWithUpdatedAuth,
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///`AUTH`
    pub webhook_type: String,
}
impl std::fmt::Display for AuthDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
