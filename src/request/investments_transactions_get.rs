use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::InvestmentsTransactionsGetRequestOptions;
/**You should use this struct via [`PlaidClient::investments_transactions_get`].

On request success, this will return a [`InvestmentsTransactionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequest {
    pub access_token: String,
    pub end_date: chrono::NaiveDate,
    pub options: Option<InvestmentsTransactionsGetRequestOptions>,
    pub start_date: chrono::NaiveDate,
}
impl FluentRequest<'_, InvestmentsTransactionsGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: InvestmentsTransactionsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, InvestmentsTransactionsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::InvestmentsTransactionsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/investments/transactions/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "end_date" : self.params.end_date }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "start_date" : self.params.start_date }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get investment transactions

The `/investments/transactions/get` endpoint allows developers to retrieve up to 24 months of user-authorized transaction data for investment accounts.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.

Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the `total_investment_transactions` response body field to fetch all available investment transactions.

Note that Investments does not have a webhook to indicate when initial transaction data has loaded (unless you use the `async_update` option). Instead, if transactions data is not ready when `/investments/transactions/get` is first called, Plaid will wait for the data. For this reason, calling `/investments/transactions/get` immediately after Link may take up to one to two minutes to return.

Data returned by the asynchronous investments extraction flow (when `async_update` is set to true) may not be immediately available to `/investments/transactions/get`. To be alerted when the data is ready to be fetched, listen for the `HISTORICAL_UPDATE` webhook. If no investments history is ready when `/investments/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentstransactionsget>.*/
    pub fn investments_transactions_get(
        &self,
        access_token: &str,
        end_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
    ) -> FluentRequest<'_, InvestmentsTransactionsGetRequest> {
        FluentRequest {
            client: self,
            params: InvestmentsTransactionsGetRequest {
                access_token: access_token.to_owned(),
                end_date,
                options: None,
                start_date,
            },
        }
    }
}
