use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::user_third_party_token_remove`].

On request success, this will return a [`UserThirdPartyTokenRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserThirdPartyTokenRemoveRequest {
    pub third_party_user_token: String,
}
impl FluentRequest<'_, UserThirdPartyTokenRemoveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, UserThirdPartyTokenRemoveRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::UserThirdPartyTokenRemoveResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/third_party_token/remove";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "third_party_user_token" : self.params.third_party_user_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Remove a third-party user token

This endpoint is used to delete a third-party user token. Once removed, the token can longer be used to access data associated with the user.

Any subsequent calls to retrieve information using the same third-party user token will result in an error stating the third-party user token does not exist.

See endpoint docs at <https://plaid.com/docs/api/users/#userthirdpartytokenremove>.*/
    pub fn user_third_party_token_remove(
        &self,
        third_party_user_token: &str,
    ) -> FluentRequest<'_, UserThirdPartyTokenRemoveRequest> {
        FluentRequest {
            client: self,
            params: UserThirdPartyTokenRemoveRequest {
                third_party_user_token: third_party_user_token.to_owned(),
            },
        }
    }
}
