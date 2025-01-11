use serde::{Serialize, Deserialize};
use super::{DocumentMetadata, PlaidError, Taxform};
///IncomeVerificationTaxformsGetResponse defines the response schema for `/income/verification/taxforms/get`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_metadata: Vec<DocumentMetadata>,
    ///Errors are identified by `error_code` and categorized by `error_type`. Use these in preference to HTTP status codes to identify and handle specific errors. HTTP status codes are set and provide the broadest categorization of errors: 4xx codes are for developer- or user-related errors, and 5xx codes are for Plaid-related errors, and the status will be 2xx in non-error cases. An Item with a non-`null` error object will only be part of an API response when calling `/item/get` to view Item status. Otherwise, error fields will be `null` if no error has occurred; if an error has occurred, an error code will be returned instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///A list of forms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub taxforms: Vec<Taxform>,
}
impl std::fmt::Display for IncomeVerificationTaxformsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
