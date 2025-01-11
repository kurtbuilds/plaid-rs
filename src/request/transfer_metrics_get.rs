use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_metrics_get`].

On request success, this will return a [`TransferMetricsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferMetricsGetRequest {
    pub originator_client_id: Option<String>,
}
impl FluentRequest<'_, TransferMetricsGetRequest> {
    ///Set the value of the originator_client_id field.
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferMetricsGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::TransferMetricsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/metrics/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(serde_json::json!({ "originator_client_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get transfer product usage metrics

Use the `/transfer/metrics/get` endpoint to view your transfer product usage metrics.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/metrics/#transfermetricsget>.*/
    pub fn transfer_metrics_get(&self) -> FluentRequest<'_, TransferMetricsGetRequest> {
        FluentRequest {
            client: self,
            params: TransferMetricsGetRequest {
                originator_client_id: None,
            },
        }
    }
}
