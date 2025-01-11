use serde::{Serialize, Deserialize};
use super::{PlaidError, WebhookEnvironmentValues};
///Fired when Plaid detects a new account. Upon receiving this webhook, you can prompt your users to share new accounts with you through [update mode](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts) (US/CA only). If the end user has opted not to share new accounts with Plaid via their institution's OAuth settings, Plaid will not detect new accounts and this webhook will not fire. For end user accounts in the EU and UK, upon receiving this webhook, you can prompt your user to re-link their account and then delete the old Item via `/item/remove`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAccountsAvailableWebhook {
    ///The Plaid environment the webhook was sent from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<WebhookEnvironmentValues>,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    ///`NEW_ACCOUNTS_AVAILABLE`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_code: Option<String>,
    ///`ITEM`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<String>,
}
impl std::fmt::Display for NewAccountsAvailableWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
