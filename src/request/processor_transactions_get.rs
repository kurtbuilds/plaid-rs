use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ProcessorTransactionsGetRequestOptions;
/**You should use this struct via [`PlaidClient::processor_transactions_get`].

On request success, this will return a [`ProcessorTransactionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTransactionsGetRequest {
    pub end_date: chrono::NaiveDate,
    pub options: Option<ProcessorTransactionsGetRequestOptions>,
    pub processor_token: String,
    pub start_date: chrono::NaiveDate,
}
impl FluentRequest<'_, ProcessorTransactionsGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: ProcessorTransactionsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTransactionsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorTransactionsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/transactions/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "end_date" : self.params.end_date }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            r = r.json(serde_json::json!({ "start_date" : self.params.start_date }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get transaction data

The `/processor/transactions/get` endpoint allows developers to receive user-authorized transaction data for credit, depository, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.  Transactions are not immutable and can also be removed altogether by the institution; a removed transaction will no longer appear in `/processor/transactions/get`.  For more details, see [Pending and posted transactions](https://plaid.com/docs/transactions/transactions-data/#pending-and-posted-transactions).

Due to the potentially large number of transactions associated with a processor token, results are paginated. Manipulate the `count` and `offset` parameters in conjunction with the `total_transactions` response body field to fetch all available transactions.

Data returned by `/processor/transactions/get` will be the data available for the processor token as of the most recent successful check for new transactions. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. To force Plaid to check for new transactions, you can use the `/processor/transactions/refresh` endpoint.

Note that data may not be immediately available to `/processor/transactions/get`. Plaid will begin to prepare transactions data upon Item link, if Link was initialized with `transactions`, or upon the first call to `/processor/transactions/get`, if it wasn't. If no transaction history is ready when `/processor/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

To receive Transactions webhooks for a processor token, set its webhook URL via the [`/processor/token/webhook/update`](https://plaid.com/docs/api/processor-partners/#processortokenwebhookupdate) endpoint.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processortransactionsget>.*/
    pub fn processor_transactions_get(
        &self,
        end_date: chrono::NaiveDate,
        processor_token: &str,
        start_date: chrono::NaiveDate,
    ) -> FluentRequest<'_, ProcessorTransactionsGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorTransactionsGetRequest {
                end_date,
                options: None,
                processor_token: processor_token.to_owned(),
                start_date,
            },
        }
    }
}
