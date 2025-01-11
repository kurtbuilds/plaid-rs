use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::SandboxIncomeWebhookFireRequestWebhookCode;
/**You should use this struct via [`PlaidClient::sandbox_income_fire_webhook`].

On request success, this will return a [`SandboxIncomeFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookRequest {
    pub item_id: String,
    pub user_id: Option<String>,
    pub verification_status: Option<String>,
    pub webhook: String,
    pub webhook_code: SandboxIncomeWebhookFireRequestWebhookCode,
}
impl FluentRequest<'_, SandboxIncomeFireWebhookRequest> {
    ///Set the value of the user_id field.
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.params.user_id = Some(user_id.to_owned());
        self
    }
    ///Set the value of the verification_status field.
    pub fn verification_status(mut self, verification_status: &str) -> Self {
        self.params.verification_status = Some(verification_status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxIncomeFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxIncomeFireWebhookResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/income/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "item_id" : self.params.item_id }));
            if let Some(ref unwrapped) = self.params.user_id {
                r = r.json(serde_json::json!({ "user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.verification_status {
                r = r.json(serde_json::json!({ "verification_status" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "webhook" : self.params.webhook }));
            r = r.json(serde_json::json!({ "webhook_code" : self.params.webhook_code }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Manually fire an Income webhook

Use the `/sandbox/income/fire_webhook` endpoint to manually trigger a Payroll or Document Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxincomefire_webhook>.*/
    pub fn sandbox_income_fire_webhook(
        &self,
        item_id: &str,
        webhook: &str,
        webhook_code: SandboxIncomeWebhookFireRequestWebhookCode,
    ) -> FluentRequest<'_, SandboxIncomeFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: SandboxIncomeFireWebhookRequest {
                item_id: item_id.to_owned(),
                user_id: None,
                verification_status: None,
                webhook: webhook.to_owned(),
                webhook_code,
            },
        }
    }
}
