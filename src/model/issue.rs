use serde::{Serialize, Deserialize};
use super::IssuesStatus;
///Information on an issue encountered with financial institutions interactions with financial institutions during Linking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    ///The creation time of the record tracking this issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///A more detailed description for the customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detailed_description: Option<String>,
    ///A list of ids of the financial institutions affected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_ids: Option<Vec<String>>,
    ///A list of names of the financial institutions affected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_names: Option<Vec<String>>,
    ///The unique identifier of the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    ///The current status of the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IssuesStatus>,
    ///A simple summary of the error for the end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
