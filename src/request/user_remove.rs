use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::user_remove`].

On request success, this will return a [`UserRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRemoveRequest {
    pub user_token: String,
}
impl FluentRequest<'_, UserRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UserRemoveRequest> {
    type Output = httpclient::InMemoryResult<crate::model::UserRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/remove";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Remove user

`/user/remove` deletes a user token and and associated information, including any Items associated with the token.
Any subsequent calls to retrieve information using the same user token will result in an error stating the user does not exist.

See endpoint docs at <https://plaid.com/docs/api/users/#userremove>.*/
    pub fn user_remove(&self, user_token: &str) -> FluentRequest<'_, UserRemoveRequest> {
        FluentRequest {
            client: self,
            params: UserRemoveRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
