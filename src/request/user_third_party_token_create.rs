use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::user_third_party_token_create`].

On request success, this will return a [`UserThirdPartyTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserThirdPartyTokenCreateRequest {
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    pub third_party_client_id: String,
    pub user_token: String,
}
impl FluentRequest<'_, UserThirdPartyTokenCreateRequest> {
    ///Set the value of the expiration_time field.
    pub fn expiration_time(
        mut self,
        expiration_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.expiration_time = Some(expiration_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, UserThirdPartyTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::UserThirdPartyTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/third_party_token/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.expiration_time {
                r = r.json(serde_json::json!({ "expiration_time" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "third_party_client_id" : self.params.third_party_client_id }
                    ),
                );
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a third-party user token

This endpoint is used to create a third-party user token. This token can be shared with and used by a specified third-party client to access data associated with the user through supported endpoints.

Ensure you store the `third_party_user_token` along with the `user_token` and `third_party_client_id`, as it is not possible to retrieve a previously created `third_party_user_token`.

See endpoint docs at <https://plaid.com/docs/api/users/#userthirdpartytokencreate>.*/
    pub fn user_third_party_token_create(
        &self,
        third_party_client_id: &str,
        user_token: &str,
    ) -> FluentRequest<'_, UserThirdPartyTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: UserThirdPartyTokenCreateRequest {
                expiration_time: None,
                third_party_client_id: third_party_client_id.to_owned(),
                user_token: user_token.to_owned(),
            },
        }
    }
}
