use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when a Check Report has failed to generate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportFailedWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`CHECK_REPORT_FAILED`
    pub webhook_code: String,
    ///`CHECK_REPORT`
    pub webhook_type: String,
}
impl std::fmt::Display for CraCheckReportFailedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
