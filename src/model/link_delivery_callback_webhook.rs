use serde::{Serialize, Deserialize};
use super::{LinkCallbackMetadata, PlaidError};
///Webhook containing metadata proxied over from Link callback e.g `onEvent`, `onExit`, `onSuccess`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkDeliveryCallbackWebhook {
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///Information related to the callback from the Hosted Link session.
    pub link_callback_metadata: LinkCallbackMetadata,
    ///The ID of the Hosted Link session.
    pub link_delivery_session_id: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub timestamp: String,
    ///`LINK_CALLBACK`
    pub webhook_code: String,
    ///`LINK_DELIVERY`
    pub webhook_type: String,
}
impl std::fmt::Display for LinkDeliveryCallbackWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
