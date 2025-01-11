use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_capabilities_get`].

On request success, this will return a [`TransferCapabilitiesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCapabilitiesGetRequest {
    pub access_token: String,
    pub account_id: String,
    pub payment_profile_token: Option<String>,
}
impl FluentRequest<'_, TransferCapabilitiesGetRequest> {
    ///Set the value of the payment_profile_token field.
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.params.payment_profile_token = Some(payment_profile_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferCapabilitiesGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferCapabilitiesGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/capabilities/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.payment_profile_token {
                r = r.json(serde_json::json!({ "payment_profile_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get RTP eligibility information of a transfer

Use the `/transfer/capabilities/get` endpoint to determine the RTP eligibility information of an account to be used with Transfer. This endpoint works on all Transfer-capable Items, including those created by `/transfer/migrate_account`. To simulate RTP eligibility in Sandbox, log in using the username `user_good` and password `pass_good` and use the first two checking and savings accounts in the "First Platypus Bank" institution (ending in 0000 or 1111), which will return `true`. Any other account will return `false`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/account-linking/#transfercapabilitiesget>.*/
    pub fn transfer_capabilities_get(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> FluentRequest<'_, TransferCapabilitiesGetRequest> {
        FluentRequest {
            client: self,
            params: TransferCapabilitiesGetRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
                payment_profile_token: None,
            },
        }
    }
}
