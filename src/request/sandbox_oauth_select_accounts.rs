use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_oauth_select_accounts`].

On request success, this will return a [`SandboxOauthSelectAccountsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsRequest {
    pub accounts: Vec<String>,
    pub oauth_state_id: String,
}
impl FluentRequest<'_, SandboxOauthSelectAccountsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxOauthSelectAccountsRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxOauthSelectAccountsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/oauth/select_accounts";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "accounts" : self.params.accounts }));
            r = r
                .json(
                    serde_json::json!({ "oauth_state_id" : self.params.oauth_state_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    ///Save the selected accounts when connecting to the Platypus Oauth institution
    pub fn sandbox_oauth_select_accounts(
        &self,
        accounts: &[&str],
        oauth_state_id: &str,
    ) -> FluentRequest<'_, SandboxOauthSelectAccountsRequest> {
        FluentRequest {
            client: self,
            params: SandboxOauthSelectAccountsRequest {
                accounts: accounts.iter().map(|&x| x.to_owned()).collect(),
                oauth_state_id: oauth_state_id.to_owned(),
            },
        }
    }
}
