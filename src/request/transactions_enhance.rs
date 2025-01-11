use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ClientProvidedRawTransaction;
/**You should use this struct via [`PlaidClient::transactions_enhance`].

On request success, this will return a [`TransactionsEnhanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsEnhanceRequest {
    pub account_type: String,
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl FluentRequest<'_, TransactionsEnhanceRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransactionsEnhanceRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransactionsEnhanceGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/v1/enhance";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "account_type" : self.params.account_type }));
            r = r.json(serde_json::json!({ "transactions" : self.params.transactions }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**enhance locally-held transaction data

The `/beta/transactions/v1/enhance` endpoint enriches raw transaction data provided directly by clients.

The product is currently in beta.*/
    pub fn transactions_enhance(
        &self,
        account_type: &str,
        transactions: Vec<ClientProvidedRawTransaction>,
    ) -> FluentRequest<'_, TransactionsEnhanceRequest> {
        FluentRequest {
            client: self,
            params: TransactionsEnhanceRequest {
                account_type: account_type.to_owned(),
                transactions,
            },
        }
    }
}
