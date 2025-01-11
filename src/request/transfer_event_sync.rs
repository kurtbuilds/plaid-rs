use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_event_sync`].

On request success, this will return a [`TransferEventSyncResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferEventSyncRequest {
    pub after_id: i64,
    pub count: Option<i64>,
}
impl FluentRequest<'_, TransferEventSyncRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferEventSyncRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferEventSyncResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/event/sync";
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
    /**Sync transfer events

`/transfer/event/sync` allows you to request up to the next 25 transfer events that happened after a specific `event_id`. Use the `/transfer/event/sync` endpoint to guarantee you have seen all transfer events.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/reading-transfers/#transfereventsync>.*/
    pub fn transfer_event_sync(
        &self,
        after_id: i64,
    ) -> FluentRequest<'_, TransferEventSyncRequest> {
        FluentRequest {
            client: self,
            params: TransferEventSyncRequest {
                after_id,
                count: None,
            },
        }
    }
}
