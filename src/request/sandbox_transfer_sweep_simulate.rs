use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_sweep_simulate`].

On request success, this will return a [`SandboxTransferSweepSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateRequest {
    pub test_clock_id: Option<String>,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, SandboxTransferSweepSimulateRequest> {
    ///Set the value of the test_clock_id field.
    pub fn test_clock_id(mut self, test_clock_id: &str) -> Self {
        self.params.test_clock_id = Some(test_clock_id.to_owned());
        self
    }
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferSweepSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferSweepSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/sweep/simulate";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(serde_json::json!({ "test_clock_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Simulate creating a sweep

Use the `/sandbox/transfer/sweep/simulate` endpoint to create a sweep and associated events in the Sandbox environment. Upon calling this endpoint, all transfers with a sweep status of `swept` will become `swept_settled`, all `posted` or `pending` transfers with a sweep status of `unswept` will become `swept`, and all `returned` transfers with a sweep status of `swept` will become `return_swept`.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersweepsimulate>.*/
    pub fn sandbox_transfer_sweep_simulate(
        &self,
    ) -> FluentRequest<'_, SandboxTransferSweepSimulateRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferSweepSimulateRequest {
                test_clock_id: None,
                webhook: None,
            },
        }
    }
}
