use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_relay_remove`].

On request success, this will return a [`CreditRelayRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayRemoveRequest {
    pub relay_token: String,
}
impl FluentRequest<'_, CreditRelayRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayRemoveRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreditRelayRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/remove";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "relay_token" : self.params.relay_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Remove relay token

The `/credit/relay/remove` endpoint allows you to invalidate a `relay_token`. The third party holding the token will no longer be able to access or refresh the reports which the `relay_token` gives access to. The original report, associated Items, and other relay tokens that provide access to the same report are not affected and will remain accessible after removing the given `relay_token`.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayremove>.*/
    pub fn credit_relay_remove(
        &self,
        relay_token: &str,
    ) -> FluentRequest<'_, CreditRelayRemoveRequest> {
        FluentRequest {
            client: self,
            params: CreditRelayRemoveRequest {
                relay_token: relay_token.to_owned(),
            },
        }
    }
}
