use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{CountryCode, InstitutionsGetByIdRequestOptions};
/**You should use this struct via [`PlaidClient::institutions_get_by_id`].

On request success, this will return a [`InstitutionsGetByIdResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequest {
    pub country_codes: Vec<CountryCode>,
    pub institution_id: String,
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
impl FluentRequest<'_, InstitutionsGetByIdRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: InstitutionsGetByIdRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InstitutionsGetByIdRequest> {
    type Output = httpclient::InMemoryResult<crate::model::InstitutionsGetByIdResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/institutions/get_by_id";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "country_codes" : self.params.country_codes }),
                );
            r = r
                .json(
                    serde_json::json!({ "institution_id" : self.params.institution_id }),
                );
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
    /**Get details of an institution

Returns a JSON response containing details on a specified financial institution currently supported by Plaid.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` to authenticate to this endpoint. The `public_key` has been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget_by_id>.*/
    pub fn institutions_get_by_id(
        &self,
        country_codes: Vec<CountryCode>,
        institution_id: &str,
    ) -> FluentRequest<'_, InstitutionsGetByIdRequest> {
        FluentRequest {
            client: self,
            params: InstitutionsGetByIdRequest {
                country_codes,
                institution_id: institution_id.to_owned(),
                options: None,
            },
        }
    }
}
