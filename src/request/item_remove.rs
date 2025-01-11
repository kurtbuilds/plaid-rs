use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_remove`].

On request success, this will return a [`ItemRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRemoveRequest {
    pub access_token: String,
}
impl FluentRequest<'_, ItemRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemRemoveRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ItemRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/remove";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Remove an Item

The `/item/remove` endpoint allows you to remove an Item. Once removed, the `access_token`, as well as any processor tokens or bank account tokens associated with the Item, is no longer valid and cannot be used to access any data that was associated with the Item.

Calling `/item/remove` is a recommended best practice when offboarding users or if a user chooses to disconnect an account linked via Plaid. For subscription products, such as Transactions, Liabilities, and Investments, calling `/item/remove` is required to end subscription billing for the Item.

In Limited Production, calling `/item/remove` does not impact the number of remaining Limited Production Items you have available.

Removing an Item does not affect any Asset Reports or Audit Copies you have already created, which will remain accessible until you remove access to them specifically using the `/asset_report/remove` endpoint.

Also note that for certain OAuth-based institutions, an Item removed via `/item/remove` may still show as an active connection in the institution's OAuth permission manager.

API versions 2019-05-29 and earlier return a `removed` boolean as part of the response.

See endpoint docs at <https://plaid.com/docs/api/items/#itemremove>.*/
    pub fn item_remove(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, ItemRemoveRequest> {
        FluentRequest {
            client: self,
            params: ItemRemoveRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
