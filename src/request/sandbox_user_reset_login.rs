use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_user_reset_login`].

On request success, this will return a [`SandboxUserResetLoginResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxUserResetLoginRequest {
    pub item_ids: Option<Vec<String>>,
    pub user_token: String,
}
impl FluentRequest<'_, SandboxUserResetLoginRequest> {
    ///Set the value of the item_ids field.
    pub fn item_ids(
        mut self,
        item_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .item_ids = Some(
            item_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SandboxUserResetLoginRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxUserResetLoginResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/user/reset_login";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.item_ids {
                r = r.json(serde_json::json!({ "item_ids" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Force item(s) for a Sandbox User into an error state

`/sandbox/user/reset_login/` functions the same as `/sandbox/item/reset_login`, but will modify Items related to a User. This endpoint forces each Item into an `ITEM_LOGIN_REQUIRED` state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's [update mode](https://plaid.com/docs/link/update-mode) flow in the Sandbox environment.  After calling `/sandbox/user/reset_login`, You can then use Plaid Link update mode to restore Items associated with the User to a good state. An `ITEM_LOGIN_REQUIRED` webhook will also be fired after a call to this endpoint, if one is associated with the Item.


In the Sandbox, Items will transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days, even if this endpoint is not called.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxuserreset_login>.*/
    pub fn sandbox_user_reset_login(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, SandboxUserResetLoginRequest> {
        FluentRequest {
            client: self,
            params: SandboxUserResetLoginRequest {
                item_ids: None,
                user_token: user_token.to_owned(),
            },
        }
    }
}
