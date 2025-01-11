use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_webhook_update`].

On request success, this will return a [`ItemWebhookUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemWebhookUpdateRequest {
    pub access_token: String,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, ItemWebhookUpdateRequest> {
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemWebhookUpdateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ItemWebhookUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/webhook/update";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
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
    /**Update Webhook URL

The POST `/item/webhook/update` allows you to update the webhook URL associated with an Item. This request triggers a [`WEBHOOK_UPDATE_ACKNOWLEDGED`](https://plaid.com/docs/api/items/#webhook_update_acknowledged) webhook to the newly specified webhook URL.

See endpoint docs at <https://plaid.com/docs/api/items/#itemwebhookupdate>.*/
    pub fn item_webhook_update(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, ItemWebhookUpdateRequest> {
        FluentRequest {
            client: self,
            params: ItemWebhookUpdateRequest {
                access_token: access_token.to_owned(),
                webhook: None,
            },
        }
    }
}
