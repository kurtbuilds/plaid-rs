use serde::{Serialize, Deserialize};
use super::{PlaidError, WebhookEnvironmentValues};
/**The `USER_ACCOUNT_REVOKED` webhook is fired when an end user has revoked access to their account on the Data Provider's portal. This webhook is currently sent only for Chase Items, but may be sent in the future for other financial institutions that allow account-level permissions revocation through their portals. Upon receiving this webhook, it is recommended to delete any Plaid-derived data you have stored that is associated with the revoked account.

If you are using Auth and receive this webhook, this webhook indicates that the TAN associated with the revoked account is no longer valid and cannot be used to create new transfers. You should not create new ACH transfers for the account that was revoked until access has been re-granted.

You can request the user to re-grant access to their account by sending them through [update mode](https://www.plaid.com/docs/link/update-mode). Alternatively, they may re-grant access directly through the Data Provider's portal.

After the user has re-granted access, Auth customers should call the auth endpoint again to obtain the new TAN.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccountRevokedWebhook {
    ///The external account ID of the affected account
    pub account_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    ///`USER_ACCOUNT_REVOKED`
    pub webhook_code: String,
    ///`ITEM`
    pub webhook_type: String,
}
impl std::fmt::Display for UserAccountRevokedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
