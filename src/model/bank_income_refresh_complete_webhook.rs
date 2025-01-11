use serde::{Serialize, Deserialize};
use super::{BankIncomeRefreshCompleteResult, WebhookEnvironmentValues};
///Fired when a refreshed bank income report has finished generating or failed to generate, triggered by calling `/credit/bank_income/refresh`. To get this webhook, subscribe via the [Dashboard](https://dashboard.plaid.com/developers/webhooks).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankIncomeRefreshCompleteWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    /**The result of the bank income refresh report generation

`SUCCESS`: The refreshed report was successfully generated and can be retrieved via `/credit/bank_income/get`.

`FAILURE`: The refreshed report failed to be generated*/
    pub result: BankIncomeRefreshCompleteResult,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`BANK_INCOME_REFRESH_COMPLETE`
    pub webhook_code: String,
    ///`INCOME`
    pub webhook_type: String,
}
impl std::fmt::Display for BankIncomeRefreshCompleteWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
