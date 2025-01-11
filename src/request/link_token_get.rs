use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::link_token_get`].

On request success, this will return a [`LinkTokenGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenGetRequest {
    pub link_token: String,
}
impl FluentRequest<'_, LinkTokenGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, LinkTokenGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::LinkTokenGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link/token/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "link_token" : self.params.link_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get Link Token

The `/link/token/get` endpoint gets information about a Link session, including all callbacks fired during the session along with their metadata, including the public token. This endpoint is used with Link flows that don't provide a public token via frontend callbacks, such as the [Hosted Link flow](https://plaid.com/docs/link/hosted-link/) and the [Multi-Item Link flow](https://plaid.com/docs/link/multi-item-link/). It also can be useful for debugging purposes.

See endpoint docs at <https://plaid.com/docs/api/link/#linktokenget>.*/
    pub fn link_token_get(
        &self,
        link_token: &str,
    ) -> FluentRequest<'_, LinkTokenGetRequest> {
        FluentRequest {
            client: self,
            params: LinkTokenGetRequest {
                link_token: link_token.to_owned(),
            },
        }
    }
}
