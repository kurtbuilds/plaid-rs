use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_recurring_cancel`].

On request success, this will return a [`TransferRecurringCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringCancelRequest {
    pub recurring_transfer_id: String,
}
impl FluentRequest<'_, TransferRecurringCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferRecurringCancelRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferRecurringCancelResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/recurring/cancel";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "recurring_transfer_id" : self.params.recurring_transfer_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Cancel a recurring transfer.

Use the `/transfer/recurring/cancel` endpoint to cancel a recurring transfer.  Scheduled transfer that hasn't been submitted to bank will be cancelled.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/recurring-transfers/#transferrecurringcancel>.*/
    pub fn transfer_recurring_cancel(
        &self,
        recurring_transfer_id: &str,
    ) -> FluentRequest<'_, TransferRecurringCancelRequest> {
        FluentRequest {
            client: self,
            params: TransferRecurringCancelRequest {
                recurring_transfer_id: recurring_transfer_id.to_owned(),
            },
        }
    }
}
