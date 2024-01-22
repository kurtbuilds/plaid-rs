use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_relay_create`].

On request success, this will return a [`CreditRelayCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayCreateRequest {
    pub report_tokens: Vec<String>,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl CreditRelayCreateRequest {}
impl FluentRequest<'_, CreditRelayCreateRequest> {
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayCreateRequest> {
    type Output = httpclient::InMemoryResult<CreditRelayCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/create";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "report_tokens" : self.params.report_tokens }));
            r = r
                .json(
                    json!({ "secondary_client_id" : self.params.secondary_client_id }),
                );
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}