use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when an entity screening status has changed, which can occur manually via the dashboard or during ongoing monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityScreeningStatusUpdatedWebhook {
    ///The ID of the associated entity screening.
    pub entity_screening_id: String,
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    ///`ENTITY_SCREENING`
    pub webhook_type: String,
}
impl std::fmt::Display for EntityScreeningStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
