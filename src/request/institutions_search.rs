use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{CountryCode, InstitutionsSearchRequestOptions, Products};
/**You should use this struct via [`PlaidClient::institutions_search`].

On request success, this will return a [`InstitutionsSearchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsSearchRequest {
    pub country_codes: Vec<CountryCode>,
    pub options: Option<InstitutionsSearchRequestOptions>,
    pub products: Option<Vec<Products>>,
    pub query: String,
}
impl FluentRequest<'_, InstitutionsSearchRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: InstitutionsSearchRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the products field.
    pub fn products(mut self, products: Vec<Products>) -> Self {
        self.params.products = Some(products);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InstitutionsSearchRequest> {
    type Output = httpclient::InMemoryResult<crate::model::InstitutionsSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/institutions/search";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "country_codes" : self.params.country_codes }),
                );
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.products {
                r = r.json(serde_json::json!({ "products" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "query" : self.params.query }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Search institutions

Returns a JSON response containing details for institutions that match the query parameters, up to a maximum of ten institutions per query.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` parameters to authenticate to this endpoint. The `public_key` parameter has since been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionssearch>.*/
    pub fn institutions_search(
        &self,
        country_codes: Vec<CountryCode>,
        query: &str,
    ) -> FluentRequest<'_, InstitutionsSearchRequest> {
        FluentRequest {
            client: self,
            params: InstitutionsSearchRequest {
                country_codes,
                options: None,
                products: None,
                query: query.to_owned(),
            },
        }
    }
}
