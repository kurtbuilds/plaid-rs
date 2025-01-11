use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_sweep_get`].

On request success, this will return a [`TransferSweepGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSweepGetRequest {
    pub sweep_id: String,
}
impl FluentRequest<'_, TransferSweepGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferSweepGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferSweepGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/sweep/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "sweep_id" : self.params.sweep_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a sweep

The `/transfer/sweep/get` endpoint fetches a sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/reading-transfers/#transfersweepget>.*/
    pub fn transfer_sweep_get(
        &self,
        sweep_id: &str,
    ) -> FluentRequest<'_, TransferSweepGetRequest> {
        FluentRequest {
            client: self,
            params: TransferSweepGetRequest {
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
}
