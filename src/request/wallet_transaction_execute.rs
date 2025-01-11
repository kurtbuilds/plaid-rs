use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{WalletTransactionAmount, WalletTransactionCounterparty};
/**You should use this struct via [`PlaidClient::wallet_transaction_execute`].

On request success, this will return a [`WalletTransactionExecuteResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionExecuteRequest {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: String,
    pub reference: String,
    pub wallet_id: String,
}
pub struct WalletTransactionExecuteRequired<'a> {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: &'a str,
    pub reference: &'a str,
    pub wallet_id: &'a str,
}
impl FluentRequest<'_, WalletTransactionExecuteRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WalletTransactionExecuteRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WalletTransactionExecuteResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/transaction/execute";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r.json(serde_json::json!({ "counterparty" : self.params.counterparty }));
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            r = r.json(serde_json::json!({ "reference" : self.params.reference }));
            r = r.json(serde_json::json!({ "wallet_id" : self.params.wallet_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Execute a transaction using an e-wallet

Execute a transaction using the specified e-wallet.
Specify the e-wallet to debit from, the counterparty to credit to, the idempotency key to prevent duplicate transactions, the amount and reference for the transaction.
Transactions will settle in seconds to several days, depending on the underlying payment rail.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionexecute>.*/
    pub fn wallet_transaction_execute(
        &self,
        args: WalletTransactionExecuteRequired,
    ) -> FluentRequest<'_, WalletTransactionExecuteRequest> {
        FluentRequest {
            client: self,
            params: WalletTransactionExecuteRequest {
                amount: args.amount,
                counterparty: args.counterparty,
                idempotency_key: args.idempotency_key.to_owned(),
                reference: args.reference.to_owned(),
                wallet_id: args.wallet_id.to_owned(),
            },
        }
    }
}
