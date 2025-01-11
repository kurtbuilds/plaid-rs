use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::IdentityGetRequestOptions;
/**You should use this struct via [`PlaidClient::identity_get`].

On request success, this will return a [`IdentityGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityGetRequest {
    pub access_token: String,
    pub options: Option<IdentityGetRequestOptions>,
}
impl FluentRequest<'_, IdentityGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: IdentityGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IdentityGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IdentityGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
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
    /**Retrieve identity data

The `/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses. Only name data is guaranteed to be returned; other fields will be empty arrays if not provided by the institution.

Note: In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29).

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identityget>.*/
    pub fn identity_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, IdentityGetRequest> {
        FluentRequest {
            client: self,
            params: IdentityGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
}
