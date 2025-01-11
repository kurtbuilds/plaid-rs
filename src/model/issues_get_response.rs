use serde::{Serialize, Deserialize};
use super::Issue;
///IssuesGetResponse defines the response schema for `/issues/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesGetResponse {
    ///Information on an issue encountered with financial institutions interactions with financial institutions during Linking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<Issue>,
    ///A unique identifier for the API request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for IssuesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
