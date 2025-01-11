use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_fire_webhook`].

On request success, this will return a [`SandboxTransferFireWebhookResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferFireWebhookRequest {
    pub webhook: String,
}
impl FluentRequest<'_, SandboxTransferFireWebhookRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferFireWebhookRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferFireWebhookResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/fire_webhook";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "webhook" : self.params.webhook }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Manually fire a Transfer webhook

Use the `/sandbox/transfer/fire_webhook` endpoint to manually trigger a `TRANSFER_EVENTS_UPDATE` webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferfire_webhook>.*/
    pub fn sandbox_transfer_fire_webhook(
        &self,
        webhook: &str,
    ) -> FluentRequest<'_, SandboxTransferFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferFireWebhookRequest {
                webhook: webhook.to_owned(),
            },
        }
    }
}
