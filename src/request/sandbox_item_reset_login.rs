use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_item_reset_login`].

On request success, this will return a [`SandboxItemResetLoginResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemResetLoginRequest {
    pub access_token: String,
}
impl FluentRequest<'_, SandboxItemResetLoginRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, SandboxItemResetLoginRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxItemResetLoginResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/item/reset_login";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Force a Sandbox Item into an error state

`/sandbox/item/reset_login/` forces an Item into an `ITEM_LOGIN_REQUIRED` state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's [update mode](https://plaid.com/docs/link/update-mode) flow in the Sandbox environment.  After calling `/sandbox/item/reset_login`, You can then use Plaid Link update mode to restore the Item to a good state. An `ITEM_LOGIN_REQUIRED` webhook will also be fired after a call to this endpoint, if one is associated with the Item.


In the Sandbox, Items will transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days, even if this endpoint is not called.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemreset_login>.*/
    pub fn sandbox_item_reset_login(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, SandboxItemResetLoginRequest> {
        FluentRequest {
            client: self,
            params: SandboxItemResetLoginRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
}
