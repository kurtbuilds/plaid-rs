use serde::{Serialize, Deserialize};
///Webhook notifications are sent only when a subscribed issue is marked as resolved. The payload contains details about the issue at the time of its resolution, focusing on the most essential information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssueResolvedWebhook {
    ///The unique identifier of the financial institution involved.
    pub institution_id: String,
    ///The name of the financial institution involved.
    pub institution_name: String,
    ///A simple description of the error for the end user.
    pub issue_description: String,
    ///The unique identifier of the connectivity issue.
    pub issue_id: String,
    ///The time when the issue was marked as resolved.
    pub issue_resolved_at: chrono::DateTime<chrono::Utc>,
    ///`ISSUE_RESOLVED`
    pub webhook_code: String,
    ///`ISSUES`
    pub webhook_type: String,
}
impl std::fmt::Display for IssueResolvedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
