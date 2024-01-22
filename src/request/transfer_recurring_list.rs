use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_recurring_list`].

On request success, this will return a [`TransferRecurringListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringListRequest {
    pub count: Option<i64>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub funding_account_id: Option<String>,
    pub offset: Option<i64>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl TransferRecurringListRequest {}
impl FluentRequest<'_, TransferRecurringListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn end_time(mut self, end_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_time = Some(end_time);
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.params.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_time = Some(start_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRecurringListRequest> {
    type Output = httpclient::InMemoryResult<TransferRecurringListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/recurring/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_time {
                r = r.json(json!({ "end_time" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.funding_account_id {
                r = r.json(json!({ "funding_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_time {
                r = r.json(json!({ "start_time" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}