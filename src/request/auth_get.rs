use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::AuthGetRequestOptions;
/**You should use this struct via [`PlaidClient::auth_get`].

On request success, this will return a [`AuthGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthGetRequest {
    pub access_token: String,
    pub options: Option<AuthGetRequestOptions>,
}
impl FluentRequest<'_, AuthGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: AuthGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AuthGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/auth/get";
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
    /**Retrieve auth data

The `/auth/get` endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.

Versioning note: In API version 2017-03-08, the schema of the `numbers` object returned by this endpoint is substantially different. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2018-05-22).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#authget>.*/
    pub fn auth_get(&self, access_token: &str) -> FluentRequest<'_, AuthGetRequest> {
        FluentRequest {
            client: self,
            params: AuthGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
}
