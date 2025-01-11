use serde::{Serialize, Deserialize};
///The webhook codes this endpoint can be used to test
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SandboxBankIncomeWebhookFireRequestWebhookCode {
    #[serde(rename = "BANK_INCOME_REFRESH_UPDATE")]
    BankIncomeRefreshUpdate,
    #[serde(rename = "BANK_INCOME_REFRESH_COMPLETE")]
    BankIncomeRefreshComplete,
}
