use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_refund_create`].

On request success, this will return a [`TransferRefundCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundCreateRequest {
    pub amount: String,
    pub idempotency_key: String,
    pub transfer_id: String,
}
impl FluentRequest<'_, TransferRefundCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRefundCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferRefundCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/refund/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            r = r.json(serde_json::json!({ "transfer_id" : self.params.transfer_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a refund

Use the `/transfer/refund/create` endpoint to create a refund for a transfer. A transfer can be refunded if the transfer was initiated in the past 180 days.

Processing of the refund will not occur until at least 4 business days following the transfer's settlement date, plus any hold/settlement delays. This 3-day window helps better protect your business from regular ACH returns. Consumer initiated returns (unauthorized returns) could still happen for about 60 days from the settlement date. If the original transfer is canceled, returned or failed, all pending refunds will automatically be canceled. Processed refunds cannot be revoked.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/refunds/#transferrefundcreate>.*/
    pub fn transfer_refund_create(
        &self,
        amount: &str,
        idempotency_key: &str,
        transfer_id: &str,
    ) -> FluentRequest<'_, TransferRefundCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferRefundCreateRequest {
                amount: amount.to_owned(),
                idempotency_key: idempotency_key.to_owned(),
                transfer_id: transfer_id.to_owned(),
            },
        }
    }
}
