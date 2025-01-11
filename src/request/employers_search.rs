use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::employers_search`].

On request success, this will return a [`EmployersSearchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersSearchRequest {
    pub products: Vec<String>,
    pub query: String,
}
impl FluentRequest<'_, EmployersSearchRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, EmployersSearchRequest> {
    type Output = httpclient::InMemoryResult<crate::model::EmployersSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/employers/search";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "products" : self.params.products }));
            r = r.json(serde_json::json!({ "query" : self.params.query }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Search employer database

`/employers/search` allows you the ability to search Plaid’s database of known employers, for use with Deposit Switch. You can use this endpoint to look up a user's employer in order to confirm that they are supported. Users with non-supported employers can then be routed out of the Deposit Switch flow.

The data in the employer database is currently limited. As the Deposit Switch and Income products progress through their respective beta periods, more employers are being regularly added. Because the employer database is frequently updated, we recommend that you do not cache or store data from this endpoint for more than a day.

See endpoint docs at <https://plaid.com/docs/api/employers/#employerssearch>.*/
    pub fn employers_search(
        &self,
        products: &[&str],
        query: &str,
    ) -> FluentRequest<'_, EmployersSearchRequest> {
        FluentRequest {
            client: self,
            params: EmployersSearchRequest {
                products: products.iter().map(|&x| x.to_owned()).collect(),
                query: query.to_owned(),
            },
        }
    }
}
