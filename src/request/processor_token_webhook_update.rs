use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_token_webhook_update`].

On request success, this will return a [`ProcessorTokenWebhookUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenWebhookUpdateRequest {
    pub processor_token: String,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, ProcessorTokenWebhookUpdateRequest> {
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTokenWebhookUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorTokenWebhookUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/token/webhook/update";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Update a processor token's webhook URL

This endpoint allows you, the processor, to update the webhook URL associated with a processor token. This request triggers a `WEBHOOK_UPDATE_ACKNOWLEDGED` webhook to the newly specified webhook URL.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processortokenwebhookupdate>.*/
    pub fn processor_token_webhook_update(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorTokenWebhookUpdateRequest> {
        FluentRequest {
            client: self,
            params: ProcessorTokenWebhookUpdateRequest {
                processor_token: processor_token.to_owned(),
                webhook: None,
            },
        }
    }
}
