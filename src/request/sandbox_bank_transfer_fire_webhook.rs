use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_bank_transfer_fire_webhook`].

On request success, this will return a [`SandboxBankTransferFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookRequest {
    pub webhook: String,
}
impl FluentRequest<'_, SandboxBankTransferFireWebhookRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxBankTransferFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxBankTransferFireWebhookResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/bank_transfer/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "webhook" : self.params.webhook }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Manually fire a Bank Transfer webhook

Use the `/sandbox/bank_transfer/fire_webhook` endpoint to manually trigger a Bank Transfers webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transferfire_webhook>.*/
    pub fn sandbox_bank_transfer_fire_webhook(
        &self,
        webhook: &str,
    ) -> FluentRequest<'_, SandboxBankTransferFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: SandboxBankTransferFireWebhookRequest {
                webhook: webhook.to_owned(),
            },
        }
    }
}
