use serde::{Serialize, Deserialize};
use super::{MonitoringInsightsStatus, WebhookEnvironmentValues};
///For each user enabled for Cash Flow Updates, this webhook will fire every 14 days with information on the status of the update. Upon receiving the webhook, call `/cra/monitoring_insights/get` to retrieve the updated insights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringInsightsWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///The reason for why insights may not be `AVAILABLE`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///Enum for the status of the insights
    pub status: MonitoringInsightsStatus,
    ///The `user_id` that the report is associated with
    pub user_id: String,
    ///`INSIGHTS_UPDATED`
    pub webhook_code: String,
    ///`CRA_MONITORING`
    pub webhook_type: String,
}
impl std::fmt::Display for MonitoringInsightsWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
