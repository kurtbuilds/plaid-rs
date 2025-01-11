use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    SandboxBankIncomeWebhookFireRequestWebhookCode,
    SandboxBankIncomeWebhookFireRequestWebhookFields,
};
/**You should use this struct via [`PlaidClient::sandbox_bank_income_fire_webhook`].

On request success, this will return a [`SandboxBankIncomeFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxBankIncomeFireWebhookRequest {
    pub webhook_code: SandboxBankIncomeWebhookFireRequestWebhookCode,
    pub webhook_fields: SandboxBankIncomeWebhookFireRequestWebhookFields,
    pub webhook_override: Option<String>,
}
impl FluentRequest<'_, SandboxBankIncomeFireWebhookRequest> {
    ///Set the value of the webhook_override field.
    pub fn webhook_override(mut self, webhook_override: &str) -> Self {
        self.params.webhook_override = Some(webhook_override.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxBankIncomeFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxBankIncomeFireWebhookResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/bank_income/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "webhook_code" : self.params.webhook_code }));
            r = r
                .json(
                    serde_json::json!({ "webhook_fields" : self.params.webhook_fields }),
                );
            if let Some(ref unwrapped) = self.params.webhook_override {
                r = r.json(serde_json::json!({ "webhook_override" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Manually fire a bank income webhook in sandbox

Use the `/sandbox/bank_income/fire_webhook` endpoint to manually trigger a Bank Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxbankincomefire_webhook>.*/
    pub fn sandbox_bank_income_fire_webhook(
        &self,
        webhook_code: SandboxBankIncomeWebhookFireRequestWebhookCode,
        webhook_fields: SandboxBankIncomeWebhookFireRequestWebhookFields,
    ) -> FluentRequest<'_, SandboxBankIncomeFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: SandboxBankIncomeFireWebhookRequest {
                webhook_code,
                webhook_fields,
                webhook_override: None,
            },
        }
    }
}
