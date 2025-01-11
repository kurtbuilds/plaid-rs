use serde::{Serialize, Deserialize};
use super::Issue;
///IssuesSearchResponse defines the response schema for `/issues/search`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuesSearchResponse {
    ///A list of issues affecting the Item, session, or request passed in, conforming to the Issues data model. An empty list indicates that no matching issues were found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
    ///A unique identifier for the API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for IssuesSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
