use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{ItemImportRequestOptions, Products, ItemImportRequestUserAuth};
/**You should use this struct via [`PlaidClient::item_import`].

On request success, this will return a [`ItemImportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemImportRequest {
    pub options: Option<ItemImportRequestOptions>,
    pub products: Vec<Products>,
    pub user_auth: ItemImportRequestUserAuth,
}
impl FluentRequest<'_, ItemImportRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: ItemImportRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ItemImportRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ItemImportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/item/import";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "products" : self.params.products }));
            r = r.json(serde_json::json!({ "user_auth" : self.params.user_auth }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Import Item

`/item/import` creates an Item via your Plaid Exchange Integration and returns an `access_token`. As part of an `/item/import` request, you will include a User ID (`user_auth.user_id`) and Authentication Token (`user_auth.auth_token`) that enable data aggregation through your Plaid Exchange API endpoints. These authentication principals are to be chosen by you.

Upon creating an Item via `/item/import`, Plaid will automatically begin an extraction of that Item through the Plaid Exchange infrastructure you have already integrated.*/
    pub fn item_import(
        &self,
        products: Vec<Products>,
        user_auth: ItemImportRequestUserAuth,
    ) -> FluentRequest<'_, ItemImportRequest> {
        FluentRequest {
            client: self,
            params: ItemImportRequest {
                options: None,
                products,
                user_auth,
            },
        }
    }
}
