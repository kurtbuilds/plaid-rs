use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::bank_transfer_sweep_list`].

On request success, this will return a [`BankTransferSweepListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferSweepListRequest {
    pub count: Option<i64>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub origination_account_id: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, BankTransferSweepListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the end_time field.
    pub fn end_time(mut self, end_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_time = Some(end_time);
        self
    }
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    ///Set the value of the start_time field.
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_time = Some(start_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferSweepListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BankTransferSweepListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/sweep/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_time {
                r = r.json(serde_json::json!({ "end_time" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_time {
                r = r.json(serde_json::json!({ "start_time" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List sweeps

The `/bank_transfer/sweep/list` endpoint fetches information about the sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweeplist>.*/
    pub fn bank_transfer_sweep_list(
        &self,
    ) -> FluentRequest<'_, BankTransferSweepListRequest> {
        FluentRequest {
            client: self,
            params: BankTransferSweepListRequest {
                count: None,
                end_time: None,
                origination_account_id: None,
                start_time: None,
            },
        }
    }
}
