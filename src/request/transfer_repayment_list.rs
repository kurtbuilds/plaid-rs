use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_repayment_list`].

On request success, this will return a [`TransferRepaymentListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRepaymentListRequest {
    pub count: Option<i64>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, TransferRepaymentListRequest> {
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
    ///Set the value of the offset field.
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    ///Set the value of the start_date field.
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRepaymentListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferRepaymentListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/repayment/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(serde_json::json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_date {
                r = r.json(serde_json::json!({ "start_date" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Lists historical repayments

The `/transfer/repayment/list` endpoint fetches repayments matching the given filters. Repayments are returned in reverse-chronological order (most recent first) starting at the given `start_time`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentlist>.*/
    pub fn transfer_repayment_list(
        &self,
    ) -> FluentRequest<'_, TransferRepaymentListRequest> {
        FluentRequest {
            client: self,
            params: TransferRepaymentListRequest {
                count: None,
                end_date: None,
                offset: None,
                start_date: None,
            },
        }
    }
}
