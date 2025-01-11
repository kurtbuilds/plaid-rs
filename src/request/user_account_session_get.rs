use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::user_account_session_get`].

On request success, this will return a [`UserAccountSessionGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccountSessionGetRequest {
    pub public_token: String,
}
impl FluentRequest<'_, UserAccountSessionGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UserAccountSessionGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::UserAccountSessionGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user_account/session/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "public_token" : self.params.public_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve User Account

Returns user permissioned account data including identity and Item access tokens.

See endpoint docs at <https://plaid.com/docs/api/products/layer/#user_accountsessionget>.*/
    pub fn user_account_session_get(
        &self,
        public_token: &str,
    ) -> FluentRequest<'_, UserAccountSessionGetRequest> {
        FluentRequest {
            client: self,
            params: UserAccountSessionGetRequest {
                public_token: public_token.to_owned(),
            },
        }
    }
}
