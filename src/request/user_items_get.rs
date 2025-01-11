use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::user_items_get`].

On request success, this will return a [`UserItemsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserItemsGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, UserItemsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UserItemsGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::UserItemsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/items/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get Items associated with a User

Returns Items associated with a User along with their corresponding statuses.

See endpoint docs at <https://plaid.com/docs/api/users/#useritemsget>.*/
    pub fn user_items_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, UserItemsGetRequest> {
        FluentRequest {
            client: self,
            params: UserItemsGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
