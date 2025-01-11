use serde::{Serialize, Deserialize};
use super::WebhookEnvironmentValues;
///Fired when new transfer events are available. Receiving this webhook indicates you should fetch the new events from `/transfer/event/sync`. If multiple transfer events occur within a single minute, only one webhook will be fired, so a single webhook instance may correspond to multiple transfer events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEventsUpdateWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    ///`TRANSFER_EVENTS_UPDATE`
    pub webhook_code: String,
    ///`TRANSFER`
    pub webhook_type: String,
}
impl std::fmt::Display for TransferEventsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
