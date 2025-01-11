use serde::{Serialize, Deserialize};
use super::PlaidErrorType;
///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaidError {
    /**In this product, a request can pertain to more than one Item. If an error is returned for such a request, `causes` will return an array of errors containing a breakdown of these errors on the individual Item level, if any can be identified.

`causes` will only be provided for the `error_type` `ASSET_REPORT_ERROR`. `causes` will also not be populated inside an error nested within a `warning` object.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causes: Option<Vec<serde_json::Value>>,
    /**A user-friendly representation of the error code. `null` if the error is not related to user action.

This may change over time and is not safe for programmatic use.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_message: Option<String>,
    ///The URL of a Plaid documentation page with more information about the error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    ///The particular error code. Safe for programmatic use.
    pub error_code: String,
    /**The specific reason for the error code. Currently, reasons are only supported OAuth-based item errors; `null` will be returned otherwise. Safe for programmatic use.

Possible values:
`OAUTH_INVALID_TOKEN`: The user’s OAuth connection to this institution has been invalidated.

`OAUTH_CONSENT_EXPIRED`: The user's access consent for this OAuth connection to this institution has expired.

`OAUTH_REVOKED_TOKEN`: The user’s OAuth connection to this institution is invalid because the user revoked their connection.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code_reason: Option<String>,
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: PlaidErrorType,
    ///A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    ///Suggested steps for resolving the error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
}
impl std::fmt::Display for PlaidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
