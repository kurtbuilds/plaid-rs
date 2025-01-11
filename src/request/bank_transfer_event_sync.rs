use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::bank_transfer_event_sync`].

On request success, this will return a [`BankTransferEventSyncResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferEventSyncRequest {
    pub after_id: i64,
    pub count: Option<i64>,
}
impl FluentRequest<'_, BankTransferEventSyncRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferEventSyncRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BankTransferEventSyncResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/event/sync";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "after_id" : self.params.after_id }));
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Sync bank transfer events

`/bank_transfer/event/sync` allows you to request up to the next 25 Plaid-initiated bank transfer events that happened after a specific `event_id`. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://www.plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#bank_transfereventsync>.*/
    pub fn bank_transfer_event_sync(
        &self,
        after_id: i64,
    ) -> FluentRequest<'_, BankTransferEventSyncRequest> {
        FluentRequest {
            client: self,
            params: BankTransferEventSyncRequest {
                after_id,
                count: None,
            },
        }
    }
}
