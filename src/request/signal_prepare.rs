use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::signal_prepare`].

On request success, this will return a [`SignalPrepareResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalPrepareRequest {
    pub access_token: String,
}
impl FluentRequest<'_, SignalPrepareRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SignalPrepareRequest> {
    type Output = httpclient::InMemoryResult<crate::model::SignalPrepareResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/signal/prepare";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Opt-in an Item to Signal

When an Item is not initialized with Signal, call `/signal/prepare` to opt-in that Item to the Signal data collection process, developing a Signal score. This should be done on Items where Signal was added in the `additional_consented_products` array but not in the `products`, `optional_products`, or `required_if_supported_products` array. If `/signal/prepare` is skipped on an Item that is not initialized with Signal, the initial call to `/signal/evaluate` on that Item will be less accurate, because Signal will have access to less data for computing the Signal score.

If run on an Item that is already initialized with Signal, this endpoint will return a 200 response and will not modify the Item.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalprepare>.*/
    pub fn signal_prepare(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, SignalPrepareRequest> {
        FluentRequest {
            client: self,
            params: SignalPrepareRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
