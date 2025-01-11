use serde::{Serialize, Deserialize};
///IssuesSubscribeResponse defines the response schema for `/issues/subscribe`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuesSubscribeResponse {
    ///A unique identifier for the API request.
    pub request_id: String,
}
impl std::fmt::Display for IssuesSubscribeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
