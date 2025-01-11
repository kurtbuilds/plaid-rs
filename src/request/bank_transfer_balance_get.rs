use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::bank_transfer_balance_get`].

On request success, this will return a [`BankTransferBalanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferBalanceGetRequest {
    pub origination_account_id: Option<String>,
}
impl FluentRequest<'_, BankTransferBalanceGetRequest> {
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BankTransferBalanceGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/balance/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get balance of your Bank Transfer account

Use the `/bank_transfer/balance/get` endpoint to see the available balance in your bank transfer account. Debit transfers increase this balance once their status is posted. Credit transfers decrease this balance when they are created.

The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.

Note that this endpoint can only be used with FBO accounts, when using Bank Transfers in the Full Service configuration. It cannot be used on your own account when using Bank Transfers in the BTS Platform configuration.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferbalanceget>.*/
    pub fn bank_transfer_balance_get(
        &self,
    ) -> FluentRequest<'_, BankTransferBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: BankTransferBalanceGetRequest {
                origination_account_id: None,
            },
        }
    }
}
