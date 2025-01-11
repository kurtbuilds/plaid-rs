use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::wallet_transaction_get`].

On request success, this will return a [`WalletTransactionGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionGetRequest {
    pub transaction_id: String,
}
impl FluentRequest<'_, WalletTransactionGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletTransactionGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::WalletTransactionGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/transaction/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "transaction_id" : self.params.transaction_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Fetch an e-wallet transaction

Fetch a specific e-wallet transaction

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionget>.*/
    pub fn wallet_transaction_get(
        &self,
        transaction_id: &str,
    ) -> FluentRequest<'_, WalletTransactionGetRequest> {
        FluentRequest {
            client: self,
            params: WalletTransactionGetRequest {
                transaction_id: transaction_id.to_owned(),
            },
        }
    }
}
