use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::profile_get`].

On request success, this will return a [`ProfileGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGetRequest {
    pub profile_token: String,
}
impl FluentRequest<'_, ProfileGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProfileGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProfileGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/profile/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "profile_token" : self.params.profile_token }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a Profile

Returns user permissioned profile data including identity and item access tokens.

See endpoint docs at <https://plaid.com/docs/api/profile/#profileget>.*/
    pub fn profile_get(
        &self,
        profile_token: &str,
    ) -> FluentRequest<'_, ProfileGetRequest> {
        FluentRequest {
            client: self,
            params: ProfileGetRequest {
                profile_token: profile_token.to_owned(),
            },
        }
    }
}
