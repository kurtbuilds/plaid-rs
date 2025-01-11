use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when the Check Report are ready to be retrieved. Once this webhook has fired, the report will be available to retrieve for 24 hours.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportReadyWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///A list of `item_ids` that is included in the Check Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`CHECK_REPORT_READY`
    pub webhook_code: String,
    ///`CHECK_REPORT`
    pub webhook_type: String,
}
impl std::fmt::Display for CraCheckReportReadyWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
