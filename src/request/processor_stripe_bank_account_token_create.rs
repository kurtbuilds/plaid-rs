use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_stripe_bank_account_token_create`].

On request success, this will return a [`ProcessorStripeBankAccountTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    pub access_token: String,
    pub account_id: String,
}
impl FluentRequest<'_, ProcessorStripeBankAccountTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorStripeBankAccountTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorStripeBankAccountTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/stripe/bank_account_token/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "account_id" : self.params.account_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create Stripe bank account token


Used to create a token suitable for sending to Stripe to enable Plaid-Stripe integrations. For a detailed guide on integrating Stripe, see [Add Stripe to your app](https://plaid.com/docs/auth/partnerships/stripe/).

Note that the Stripe bank account token is a one-time use token. To store bank account information for later use, you can use a Stripe customer object and create an associated bank account from the token, or you can use a Stripe Custom account and create an associated external bank account from the token. This bank account information should work indefinitely, unless the user's bank account information changes or they revoke Plaid's permissions to access their account. Stripe bank account information cannot be modified once the bank account token has been created. If you ever need to change the bank account details used by Stripe for a specific customer, have the user go through Link again and create a new bank account token from the new `access_token`.

To revoke a bank account token, the entire underlying access token must be revoked using `/item/remove`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorstripebank_account_tokencreate>.*/
    pub fn processor_stripe_bank_account_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> FluentRequest<'_, ProcessorStripeBankAccountTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: ProcessorStripeBankAccountTokenCreateRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
            },
        }
    }
}
