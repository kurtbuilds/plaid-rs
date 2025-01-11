use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_access_token_invalidate`].

On request success, this will return a [`ItemAccessTokenInvalidateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateRequest {
    pub access_token: String,
}
impl FluentRequest<'_, ItemAccessTokenInvalidateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ItemAccessTokenInvalidateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ItemAccessTokenInvalidateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/access_token/invalidate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Invalidate access_token

By default, the `access_token` associated with an Item does not expire and should be stored in a persistent, secure manner.

You can use the `/item/access_token/invalidate` endpoint to rotate the `access_token` associated with an Item. The endpoint returns a new `access_token` and immediately invalidates the previous `access_token`.


See endpoint docs at <https://plaid.com/docs/api/items/#itemaccess_tokeninvalidate>.*/
    pub fn item_access_token_invalidate(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, ItemAccessTokenInvalidateRequest> {
        FluentRequest {
            client: self,
            params: ItemAccessTokenInvalidateRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
