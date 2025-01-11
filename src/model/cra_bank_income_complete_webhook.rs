use serde::{Serialize, Deserialize};
use super::{CraBankIncomeCompleteResult, WebhookEnvironmentValues};
///Fired when a bank income report has finished generating or failed to generate, triggered by calling `/cra/bank_income/get`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBankIncomeCompleteWebhook {
    ///The Plaid environment the webhook was sent from
    pub environment: WebhookEnvironmentValues,
    /**The result of the bank income report generation

`SUCCESS`: The bank income report was successfully generated and can be retrieved via `/cra/bank_income/get`.

`FAILURE`: The bank income report failed to be generated*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<CraBankIncomeCompleteResult>,
    ///The `user_id` corresponding to the user the webhook has fired for.
    pub user_id: String,
    ///`BANK_INCOME_COMPLETE`
    pub webhook_code: String,
    ///`CRA_INCOME`
    pub webhook_type: String,
}
impl std::fmt::Display for CraBankIncomeCompleteWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
