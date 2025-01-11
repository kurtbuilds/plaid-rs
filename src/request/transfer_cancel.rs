use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_cancel`].

On request success, this will return a [`TransferCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCancelRequest {
    pub originator_client_id: Option<String>,
    pub transfer_id: String,
}
impl FluentRequest<'_, TransferCancelRequest> {
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferCancelRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/cancel";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "transfer_id" : self.params.transfer_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Cancel a transfer

Use the `/transfer/cancel` endpoint to cancel a transfer.  A transfer is eligible for cancellation if the `cancellable` property returned by `/transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/initiating-transfers/#transfercancel>.*/
    pub fn transfer_cancel(
        &self,
        transfer_id: &str,
    ) -> FluentRequest<'_, TransferCancelRequest> {
        FluentRequest {
            client: self,
            params: TransferCancelRequest {
                originator_client_id: None,
                transfer_id: transfer_id.to_owned(),
            },
        }
    }
}
