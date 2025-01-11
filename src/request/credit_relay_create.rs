use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_relay_create`].

On request success, this will return a [`CreditRelayCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayCreateRequest {
    pub report_tokens: Vec<String>,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, CreditRelayCreateRequest> {
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreditRelayCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "report_tokens" : self.params.report_tokens }),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "secondary_client_id" : self.params.secondary_client_id }
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
    /**Create a relay token to share an Asset Report with a partner client

Plaid can share an Asset Report directly with a participating third party on your behalf. The shared Asset Report is the exact same Asset Report originally created in `/asset_report/create`.

To grant a third party access to an Asset Report, use the `/credit/relay/create` endpoint to create a `relay_token` and then pass that token to your third party. Each third party has its own `secondary_client_id`; for example, `ce5bd328dcd34123456`. You'll need to create a separate `relay_token` for each third party that needs access to the report on your behalf.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelaycreate>.*/
    pub fn credit_relay_create(
        &self,
        report_tokens: &[&str],
        secondary_client_id: &str,
    ) -> FluentRequest<'_, CreditRelayCreateRequest> {
        FluentRequest {
            client: self,
            params: CreditRelayCreateRequest {
                report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
                secondary_client_id: secondary_client_id.to_owned(),
                webhook: None,
            },
        }
    }
}
