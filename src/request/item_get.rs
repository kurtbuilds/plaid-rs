use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::item_get`].

On request success, this will return a [`ItemGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGetRequest {
    pub access_token: String,
}
impl FluentRequest<'_, ItemGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ItemGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve an Item

Returns information about the status of an Item.

See endpoint docs at <https://plaid.com/docs/api/items/#itemget>.*/
    pub fn item_get(&self, access_token: &str) -> FluentRequest<'_, ItemGetRequest> {
        FluentRequest {
            client: self,
            params: ItemGetRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
