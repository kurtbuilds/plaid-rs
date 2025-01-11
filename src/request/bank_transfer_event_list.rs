use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    BankTransferEventListBankTransferType, BankTransferEventListDirection,
    BankTransferEventType,
};
/**You should use this struct via [`PlaidClient::bank_transfer_event_list`].

On request success, this will return a [`BankTransferEventListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEventListRequest {
    pub account_id: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub bank_transfer_type: Option<BankTransferEventListBankTransferType>,
    pub count: Option<i64>,
    pub direction: Option<BankTransferEventListDirection>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub event_types: Option<Vec<BankTransferEventType>>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, BankTransferEventListRequest> {
    ///Set the value of the account_id field.
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.params.account_id = Some(account_id.to_owned());
        self
    }
    ///Set the value of the bank_transfer_id field.
    pub fn bank_transfer_id(mut self, bank_transfer_id: &str) -> Self {
        self.params.bank_transfer_id = Some(bank_transfer_id.to_owned());
        self
    }
    ///Set the value of the bank_transfer_type field.
    pub fn bank_transfer_type(
        mut self,
        bank_transfer_type: BankTransferEventListBankTransferType,
    ) -> Self {
        self.params.bank_transfer_type = Some(bank_transfer_type);
        self
    }
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the direction field.
    pub fn direction(mut self, direction: BankTransferEventListDirection) -> Self {
        self.params.direction = Some(direction);
        self
    }
    ///Set the value of the end_date field.
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_date = Some(end_date);
        self
    }
    ///Set the value of the event_types field.
    pub fn event_types(mut self, event_types: Vec<BankTransferEventType>) -> Self {
        self.params.event_types = Some(event_types);
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferEventListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BankTransferEventListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/event/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.account_id {
                r = r.json(serde_json::json!({ "account_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.bank_transfer_id {
                r = r.json(serde_json::json!({ "bank_transfer_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.bank_transfer_type {
                r = r.json(serde_json::json!({ "bank_transfer_type" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.direction {
                r = r.json(serde_json::json!({ "direction" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_date {
                r = r.json(serde_json::json!({ "end_date" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.event_types {
                r = r.json(serde_json::json!({ "event_types" : unwrapped }));
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
    /**List bank transfer events

Use the `/bank_transfer/event/list` endpoint to get a list of Plaid-initiated ACH or bank transfer events based on specified filter criteria. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/auth#bank_transfereventlist>.*/
    pub fn bank_transfer_event_list(
        &self,
    ) -> FluentRequest<'_, BankTransferEventListRequest> {
        FluentRequest {
            client: self,
            params: BankTransferEventListRequest {
                account_id: None,
                bank_transfer_id: None,
                bank_transfer_type: None,
                count: None,
                direction: None,
                end_date: None,
                event_types: None,
                offset: None,
                origination_account_id: None,
                start_date: None,
            },
        }
    }
}
