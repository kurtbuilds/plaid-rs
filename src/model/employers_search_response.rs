use serde::{Serialize, Deserialize};
use super::Employer;
///EmployersSearchResponse defines the response schema for `/employers/search`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmployersSearchResponse {
    ///A list of employers matching the search criteria.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub employers: Vec<Employer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EmployersSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
