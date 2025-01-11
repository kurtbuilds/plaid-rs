use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::bank_transfer_sweep_get`].

On request success, this will return a [`BankTransferSweepGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferSweepGetRequest {
    pub sweep_id: String,
}
impl FluentRequest<'_, BankTransferSweepGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferSweepGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BankTransferSweepGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/bank_transfer/sweep/get";
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

The `/bank_transfer/sweep/get` endpoint fetches information about the sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweepget>.*/
    pub fn bank_transfer_sweep_get(
        &self,
        sweep_id: &str,
    ) -> FluentRequest<'_, BankTransferSweepGetRequest> {
        FluentRequest {
            client: self,
            params: BankTransferSweepGetRequest {
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
}
