use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WalletTransactionListRequestOptions;
/**You should use this struct via [`PlaidClient::wallet_transaction_list`].

On request success, this will return a [`WalletTransactionListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionListRequest {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub options: Option<WalletTransactionListRequestOptions>,
    pub wallet_id: String,
}
impl FluentRequest<'_, WalletTransactionListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the options field.
    pub fn options(mut self, options: WalletTransactionListRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletTransactionListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WalletTransactionListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/transaction/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "wallet_id" : self.params.wallet_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List e-wallet transactions

This endpoint lists the latest transactions of the specified e-wallet. Transactions are returned in descending order by the `created_at` time.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionlist>.*/
    pub fn wallet_transaction_list(
        &self,
        wallet_id: &str,
    ) -> FluentRequest<'_, WalletTransactionListRequest> {
        FluentRequest {
            client: self,
            params: WalletTransactionListRequest {
                count: None,
                cursor: None,
                options: None,
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
}
