use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::BankTransferDirection;
/**You should use this struct via [`PlaidClient::bank_transfer_list`].

On request success, this will return a [`BankTransferListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferListRequest {
    pub count: Option<i64>,
    pub direction: Option<BankTransferDirection>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, BankTransferListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the direction field.
    pub fn direction(mut self, direction: BankTransferDirection) -> Self {
        self.params.direction = Some(direction);
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
    ///Set the value of the origination_account_id field.
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.params.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    ///Set the value of the start_date field.
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_date = Some(start_date);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BankTransferListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.direction {
                r = r.json(serde_json::json!({ "direction" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(serde_json::json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.origination_account_id {
                r = r.json(serde_json::json!({ "origination_account_id" : unwrapped }));
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
    /**List bank transfers

Use the `/bank_transfer/list` endpoint to see a list of all your bank transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired bank transfers.


See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferlist>.*/
    pub fn bank_transfer_list(&self) -> FluentRequest<'_, BankTransferListRequest> {
        FluentRequest {
            client: self,
            params: BankTransferListRequest {
                count: None,
                direction: None,
                end_date: None,
                offset: None,
                origination_account_id: None,
                start_date: None,
            },
        }
    }
}
