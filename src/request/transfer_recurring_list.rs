use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
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
impl FluentRequest<'_, TransferRecurringListRequest> {
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
    ///Set the value of the start_time field.
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_time = Some(start_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRecurringListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferRecurringListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/recurring/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_time {
                r = r.json(serde_json::json!({ "end_time" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.funding_account_id {
                r = r.json(serde_json::json!({ "funding_account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
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
    /**List recurring transfers

Use the `/transfer/recurring/list` endpoint to see a list of all your recurring transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired recurring transfers.


See endpoint docs at <https://plaid.com/docs/api/products/transfer/recurring-transfers/#transferrecurringlist>.*/
    pub fn transfer_recurring_list(
        &self,
    ) -> FluentRequest<'_, TransferRecurringListRequest> {
        FluentRequest {
            client: self,
            params: TransferRecurringListRequest {
                count: None,
                end_time: None,
                funding_account_id: None,
                offset: None,
                start_time: None,
            },
        }
    }
}
