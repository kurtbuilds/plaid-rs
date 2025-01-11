use serde::{Serialize, Deserialize};
///The webhook codes that can be fired by this test endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SandboxIncomeWebhookFireRequestWebhookCode {
    #[serde(rename = "INCOME_VERIFICATION")]
    IncomeVerification,
    #[serde(rename = "INCOME_VERIFICATION_RISK_SIGNALS")]
    IncomeVerificationRiskSignals,
}
