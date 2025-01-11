use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ProfileNetworkStatusGetUser;
/**You should use this struct via [`PlaidClient::profile_network_status_get`].

On request success, this will return a [`ProfileNetworkStatusGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileNetworkStatusGetRequest {
    pub user: ProfileNetworkStatusGetUser,
}
impl FluentRequest<'_, ProfileNetworkStatusGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProfileNetworkStatusGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProfileNetworkStatusGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/profile/network_status/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Check a user's Plaid Network status

The `/profile/network_status/get` endpoint can be used to check whether Plaid has a matching profile
for the user.

See endpoint docs at <https://plaid.com/docs/api/profile/#networkstatusget>.*/
    pub fn profile_network_status_get(
        &self,
        user: ProfileNetworkStatusGetUser,
    ) -> FluentRequest<'_, ProfileNetworkStatusGetRequest> {
        FluentRequest {
            client: self,
            params: ProfileNetworkStatusGetRequest {
                user,
            },
        }
    }
}
