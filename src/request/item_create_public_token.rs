use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_create_public_token`].

On request success, this will return a [`ItemPublicTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCreatePublicTokenRequest {
    pub access_token: String,
}
impl FluentRequest<'_, ItemCreatePublicTokenRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemCreatePublicTokenRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ItemPublicTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/public_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create public token

Note: As of July 2020, the `/item/public_token/create` endpoint is deprecated. Instead, use `/link/token/create` with an `access_token` to create a Link token for use with [update mode](https://plaid.com/docs/link/update-mode).

If you need your user to take action to restore or resolve an error associated with an Item, generate a public token with the `/item/public_token/create` endpoint and then initialize Link with that `public_token`.

A `public_token` is one-time use and expires after 30 minutes. You use a `public_token` to initialize Link in [update mode](https://plaid.com/docs/link/update-mode) for a particular Item. You can generate a `public_token` for an Item even if you did not use Link to create the Item originally.

The `/item/public_token/create` endpoint is **not** used to create your initial `public_token`. If you have not already received an `access_token` for a specific Item, use Link to obtain your `public_token` instead. See the [Quickstart](https://plaid.com/docs/quickstart) for more information.

See endpoint docs at <https://plaid.com/docs/api/link/#itempublic_tokencreate>.*/
    pub fn item_create_public_token(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, ItemCreatePublicTokenRequest> {
        FluentRequest {
            client: self,
            params: ItemCreatePublicTokenRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
