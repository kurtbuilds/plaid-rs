use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::link_oauth_correlation_id_exchange`].

On request success, this will return a [`LinkOAuthCorrelationIdExchangeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkOauthCorrelationIdExchangeRequest {
    pub link_correlation_id: String,
}
impl FluentRequest<'_, LinkOauthCorrelationIdExchangeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, LinkOauthCorrelationIdExchangeRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::LinkOAuthCorrelationIdExchangeResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link/oauth/correlation_id/exchange";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "link_correlation_id" : self.params.link_correlation_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Exchange the Link Correlation Id for a Link Token

Exchange an OAuth `link_correlation_id` for the corresponding `link_token`. The `link_correlation_id` is only available for 'payment_initiation' products and is provided to the client via the OAuth `redirect_uri` as a query parameter.
The `link_correlation_id` is ephemeral and expires in a brief period, after which it can no longer be exchanged for the 'link_token'.

See endpoint docs at <https://plaid.com/docs/api/oauth/#linkcorrelationid>.*/
    pub fn link_oauth_correlation_id_exchange(
        &self,
        link_correlation_id: &str,
    ) -> FluentRequest<'_, LinkOauthCorrelationIdExchangeRequest> {
        FluentRequest {
            client: self,
            params: LinkOauthCorrelationIdExchangeRequest {
                link_correlation_id: link_correlation_id.to_owned(),
            },
        }
    }
}
