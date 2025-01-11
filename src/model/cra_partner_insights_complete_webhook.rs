use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when a partner insights report has finished generating and results are available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraPartnerInsightsCompleteWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`PARTNER_INSIGHTS_COMPLETE`
    pub webhook_code: String,
    ///`CRA_INSIGHTS`
    pub webhook_type: String,
}
impl std::fmt::Display for CraPartnerInsightsCompleteWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
