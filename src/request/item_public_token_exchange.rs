use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_public_token_exchange`].

On request success, this will return a [`ItemPublicTokenExchangeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeRequest {
    pub public_token: String,
}
impl FluentRequest<'_, ItemPublicTokenExchangeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ItemPublicTokenExchangeRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ItemPublicTokenExchangeResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/public_token/exchange";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "public_token" : self.params.public_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Exchange public token for an access token

Exchange a Link `public_token` for an API `access_token`. Link hands off the `public_token` client-side via the `onSuccess` callback once a user has successfully created an Item. The `public_token` is ephemeral and expires after 30 minutes. An `access_token` does not expire, but can be revoked by calling `/item/remove`.

The response also includes an `item_id` that should be stored with the `access_token`. The `item_id` is used to identify an Item in a webhook. The `item_id` can also be retrieved by making an `/item/get` request.

See endpoint docs at <https://plaid.com/docs/api/items/#itempublic_tokenexchange>.*/
    pub fn item_public_token_exchange(
        &self,
        public_token: &str,
    ) -> FluentRequest<'_, ItemPublicTokenExchangeRequest> {
        FluentRequest {
            client: self,
            params: ItemPublicTokenExchangeRequest {
                public_token: public_token.to_owned(),
            },
        }
    }
}
