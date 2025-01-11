use serde::{Serialize, Deserialize};
use super::{LinkSessionExitMetadata, PlaidError};
///An object representing an [onExit](https://plaid.com/docs/link/web/#onexit) callback from Link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSessionExit {
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///Displayed if a user exits Link without successfully linking an Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LinkSessionExitMetadata>,
}
impl std::fmt::Display for LinkSessionExit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
