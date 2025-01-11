use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::categories_get`].

On request success, this will return a [`CategoriesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoriesGetRequest {
    pub body: serde_json::Value,
}
impl FluentRequest<'_, CategoriesGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CategoriesGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CategoriesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/categories/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get categories

Send a request to the `/categories/get` endpoint to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

All implementations are recommended to use the newer `personal_finance_category` taxonomy instead of the older `category` taxonomy supported by this endpoint. The [`personal_finance_category taxonomy` CSV file](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) is available for download and is not accessible via API.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#categoriesget>.*/
    pub fn categories_get(
        &self,
        body: serde_json::Value,
    ) -> FluentRequest<'_, CategoriesGetRequest> {
        FluentRequest {
            client: self,
            params: CategoriesGetRequest { body },
        }
    }
}
