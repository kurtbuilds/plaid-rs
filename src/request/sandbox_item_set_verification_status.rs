use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_item_set_verification_status`].

On request success, this will return a [`SandboxItemSetVerificationStatusResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusRequest {
    pub access_token: String,
    pub account_id: String,
    pub verification_status: String,
}
impl FluentRequest<'_, SandboxItemSetVerificationStatusRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxItemSetVerificationStatusRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxItemSetVerificationStatusResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/item/set_verification_status";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            r = r
                .json(
                    serde_json::json!(
                        { "verification_status" : self.params.verification_status }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Set verification status for Sandbox account

The `/sandbox/item/set_verification_status` endpoint can be used to change the verification status of an Item in in the Sandbox in order to simulate the Automated Micro-deposit flow.

For more information on testing Automated Micro-deposits in Sandbox, see [Auth full coverage testing](https://plaid.com/docs/auth/coverage/testing#).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemset_verification_status>.*/
    pub fn sandbox_item_set_verification_status(
        &self,
        access_token: &str,
        account_id: &str,
        verification_status: &str,
    ) -> FluentRequest<'_, SandboxItemSetVerificationStatusRequest> {
        FluentRequest {
            client: self,
            params: SandboxItemSetVerificationStatusRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
                verification_status: verification_status.to_owned(),
            },
        }
    }
}
