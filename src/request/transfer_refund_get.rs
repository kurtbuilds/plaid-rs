use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_refund_get`].

On request success, this will return a [`TransferRefundGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundGetRequest {
    pub refund_id: String,
}
impl FluentRequest<'_, TransferRefundGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRefundGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferRefundGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/refund/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "refund_id" : self.params.refund_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a refund

The `/transfer/refund/get` endpoint fetches information about the refund corresponding to the given `refund_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/refunds/#transferrefundget>.*/
    pub fn transfer_refund_get(
        &self,
        refund_id: &str,
    ) -> FluentRequest<'_, TransferRefundGetRequest> {
        FluentRequest {
            client: self,
            params: TransferRefundGetRequest {
                refund_id: refund_id.to_owned(),
            },
        }
    }
}
