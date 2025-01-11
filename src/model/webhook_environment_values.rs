use serde::{Serialize, Deserialize};
///The Plaid environment the webhook was sent from
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebhookEnvironmentValues {
    #[serde(rename = "sandbox")]
    Sandbox,
    #[serde(rename = "production")]
    Production,
}
