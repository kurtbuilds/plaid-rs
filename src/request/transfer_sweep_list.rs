use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{SweepStatus, SweepTrigger};
/**You should use this struct via [`PlaidClient::transfer_sweep_list`].

On request success, this will return a [`TransferSweepListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSweepListRequest {
    pub amount: Option<String>,
    pub count: Option<i64>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub funding_account_id: Option<String>,
    pub offset: Option<i64>,
    pub originator_client_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<SweepStatus>,
    pub transfer_id: Option<String>,
    pub trigger: Option<SweepTrigger>,
}
impl FluentRequest<'_, TransferSweepListRequest> {
    ///Set the value of the amount field.
    pub fn amount(mut self, amount: &str) -> Self {
        self.params.amount = Some(amount.to_owned());
        self
    }
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the end_date field.
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_date = Some(end_date);
        self
    }
    ///Set the value of the funding_account_id field.
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    ///Set the value of the offset field.
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    ///Set the value of the start_date field.
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
    ///Set the value of the status field.
    pub fn status(mut self, status: SweepStatus) -> Self {
        self.params.status = Some(status);
        self
    }
    ///Set the value of the transfer_id field.
    pub fn transfer_id(mut self, transfer_id: &str) -> Self {
        self.params.transfer_id = Some(transfer_id.to_owned());
        self
    }
    ///Set the value of the trigger field.
    pub fn trigger(mut self, trigger: SweepTrigger) -> Self {
        self.params.trigger = Some(trigger);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferSweepListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferSweepListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/sweep/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.amount {
                r = r.json(serde_json::json!({ "amount" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(serde_json::json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.funding_account_id {
                r = r.json(serde_json::json!({ "funding_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_date {
                r = r.json(serde_json::json!({ "start_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.status {
                r = r.json(serde_json::json!({ "status" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transfer_id {
                r = r.json(serde_json::json!({ "transfer_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.trigger {
                r = r.json(serde_json::json!({ "trigger" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List sweeps

The `/transfer/sweep/list` endpoint fetches sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/reading-transfers/#transfersweeplist>.*/
    pub fn transfer_sweep_list(&self) -> FluentRequest<'_, TransferSweepListRequest> {
        FluentRequest {
            client: self,
            params: TransferSweepListRequest {
                amount: None,
                count: None,
                end_date: None,
                funding_account_id: None,
                offset: None,
                originator_client_id: None,
                start_date: None,
                status: None,
                transfer_id: None,
                trigger: None,
            },
        }
    }
}
