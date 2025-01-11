use serde::{Serialize, Deserialize};
use super::{PlaidError, WebhookEnvironmentValues};
/**The `USER_PERMISSION_REVOKED` webhook may be fired when an end user has revoked the permission that they previously granted to access an Item. If the end user revoked their permissions through Plaid (such as via the Plaid Portal or by contacting Plaid Support), the webhook will fire. If the end user revoked their permissions directly through the institution, this webhook may not always fire, since some institutions’ consent portals do not trigger this webhook. Upon receiving this webhook, it is recommended to delete any stored data from Plaid associated with the Item. To restore the Item, it can be sent through [update mode](https://plaid.com/docs/link/update-mode).

Note that when working with tokenized account numbers with Auth or Transfer, the account number provided by Plaid will no longer work for creating transfers once user permission has been revoked.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissionRevokedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`USER_PERMISSION_REVOKED`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for UserPermissionRevokedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
