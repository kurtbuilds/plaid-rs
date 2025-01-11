use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_payment_profile_reset_login`].

On request success, this will return a [`SandboxPaymentProfileResetLoginResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPaymentProfileResetLoginRequest {
    pub payment_profile_token: String,
}
impl FluentRequest<'_, SandboxPaymentProfileResetLoginRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxPaymentProfileResetLoginRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxPaymentProfileResetLoginResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/payment_profile/reset_login";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "payment_profile_token" : self.params.payment_profile_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Reset the login of a Payment Profile

`/sandbox/payment_profile/reset_login/` forces a Payment Profile into a state where the login is no longer valid. This makes it easy to test update mode for Payment Profile in the Sandbox environment.

 After calling `/sandbox/payment_profile/reset_login`, calls to the `/transfer/authorization/create` with the Payment Profile will result in a `decision_rationale` `PAYMENT_PROFILE_LOGIN_REQUIRED`. You can then use update mode for Payment Profile to restore it into a good state.

 In order to invoke this endpoint, you must first [create a Payment Profile](https://plaid.com/docs/transfer/add-to-app/#create-a-payment-profile-optional) and [go through the Link flow](https://plaid.com/docs/transfer/add-to-app/#create-a-link-token).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpayment_profilereset_login>.*/
    pub fn sandbox_payment_profile_reset_login(
        &self,
        payment_profile_token: &str,
    ) -> FluentRequest<'_, SandboxPaymentProfileResetLoginRequest> {
        FluentRequest {
            client: self,
            params: SandboxPaymentProfileResetLoginRequest {
                payment_profile_token: payment_profile_token.to_owned(),
            },
        }
    }
}
