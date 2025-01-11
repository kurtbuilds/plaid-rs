use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{IdentityMatchRequestOptions, IdentityMatchUser};
/**You should use this struct via [`PlaidClient::identity_match`].

On request success, this will return a [`IdentityMatchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMatchRequest {
    pub access_token: String,
    pub options: Option<IdentityMatchRequestOptions>,
    pub user: Option<IdentityMatchUser>,
}
impl FluentRequest<'_, IdentityMatchRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: IdentityMatchRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the user field.
    pub fn user(mut self, user: IdentityMatchUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IdentityMatchRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IdentityMatchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity/match";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(serde_json::json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve identity match score

The `/identity/match` endpoint generates a match score, which indicates how well the provided identity data matches the identity information on file with the account holder's financial institution.

Fields within the `balances` object will always be null when retrieved by `/identity/match`. Instead, use the free `/accounts/get` endpoint to request balance cached data, or `/accounts/balance/get` for real-time data.

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identitymatch>.*/
    pub fn identity_match(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, IdentityMatchRequest> {
        FluentRequest {
            client: self,
            params: IdentityMatchRequest {
                access_token: access_token.to_owned(),
                options: None,
                user: None,
            },
        }
    }
}
