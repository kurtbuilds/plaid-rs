use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_refund_cancel`].

On request success, this will return a [`TransferRefundCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundCancelRequest {
    pub refund_id: String,
}
impl FluentRequest<'_, TransferRefundCancelRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRefundCancelRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferRefundCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/refund/cancel";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "refund_id" : self.params.refund_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Cancel a refund

Use the `/transfer/refund/cancel` endpoint to cancel a refund.  A refund is eligible for cancellation if it has not yet been submitted to the payment network.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/refunds/#transferrefundcancel>.*/
    pub fn transfer_refund_cancel(
        &self,
        refund_id: &str,
    ) -> FluentRequest<'_, TransferRefundCancelRequest> {
        FluentRequest {
            client: self,
            params: TransferRefundCancelRequest {
                refund_id: refund_id.to_owned(),
            },
        }
    }
}
