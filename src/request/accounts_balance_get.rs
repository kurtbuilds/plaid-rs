use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    AccountsBalanceGetRequestOptions, AccountsBalanceGetRequestPaymentDetails,
};
/**You should use this struct via [`PlaidClient::accounts_balance_get`].

On request success, this will return a [`AccountsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequest {
    pub access_token: String,
    pub options: Option<AccountsBalanceGetRequestOptions>,
    pub payment_details: Option<AccountsBalanceGetRequestPaymentDetails>,
}
impl FluentRequest<'_, AccountsBalanceGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: AccountsBalanceGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the payment_details field.
    pub fn payment_details(
        mut self,
        payment_details: AccountsBalanceGetRequestPaymentDetails,
    ) -> Self {
        self.params.payment_details = Some(payment_details);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AccountsBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AccountsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/accounts/balance/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payment_details {
                r = r.json(serde_json::json!({ "payment_details" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve real-time balance data

The `/accounts/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints, such as `/accounts/get`, return a balance object, only `/accounts/balance/get` forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, `balance` itself is not a product that can be used to initialize Link. As this endpoint triggers a synchronous request for fresh data, latency may be higher than for other Plaid endpoints (typically less than 10 seconds, but occasionally up to 30 seconds or more); if you encounter errors, you may find it necessary to adjust your timeout period when making requests.

See endpoint docs at <https://plaid.com/docs/api/products/balance/#accountsbalanceget>.*/
    pub fn accounts_balance_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, AccountsBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: AccountsBalanceGetRequest {
                access_token: access_token.to_owned(),
                options: None,
                payment_details: None,
            },
        }
    }
}
