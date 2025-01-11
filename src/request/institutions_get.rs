use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{CountryCode, InstitutionsGetRequestOptions};
/**You should use this struct via [`PlaidClient::institutions_get`].

On request success, this will return a [`InstitutionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsGetRequest {
    pub count: i64,
    pub country_codes: Vec<CountryCode>,
    pub offset: i64,
    pub options: Option<InstitutionsGetRequestOptions>,
}
impl FluentRequest<'_, InstitutionsGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: InstitutionsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InstitutionsGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::InstitutionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/institutions/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "count" : self.params.count }));
            r = r
                .json(
                    serde_json::json!({ "country_codes" : self.params.country_codes }),
                );
            r = r.json(serde_json::json!({ "offset" : self.params.offset }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get details of all supported institutions

Returns a JSON response containing details on all financial institutions currently supported by Plaid. Because Plaid supports thousands of institutions, results are paginated.

If there is no overlap between an institution’s enabled products and a client’s enabled products, then the institution will be filtered out from the response. As a result, the number of institutions returned may not match the count specified in the call.

See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget>.*/
    pub fn institutions_get(
        &self,
        count: i64,
        country_codes: Vec<CountryCode>,
        offset: i64,
    ) -> FluentRequest<'_, InstitutionsGetRequest> {
        FluentRequest {
            client: self,
            params: InstitutionsGetRequest {
                count,
                country_codes,
                offset,
                options: None,
            },
        }
    }
}
