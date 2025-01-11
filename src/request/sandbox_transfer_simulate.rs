use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferFailure;
/**You should use this struct via [`PlaidClient::sandbox_transfer_simulate`].

On request success, this will return a [`SandboxTransferSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferSimulateRequest {
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub test_clock_id: Option<String>,
    pub transfer_id: String,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, SandboxTransferSimulateRequest> {
    ///Set the value of the failure_reason field.
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
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
for FluentRequest<'a, SandboxTransferSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/simulate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "event_type" : self.params.event_type }));
            if let Some(ref unwrapped) = self.params.failure_reason {
                r = r.json(serde_json::json!({ "failure_reason" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.test_clock_id {
                r = r.json(serde_json::json!({ "test_clock_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "transfer_id" : self.params.transfer_id }));
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
    /**Simulate a transfer event in Sandbox

Use the `/sandbox/transfer/simulate` endpoint to simulate a transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersimulate>.*/
    pub fn sandbox_transfer_simulate(
        &self,
        event_type: &str,
        transfer_id: &str,
    ) -> FluentRequest<'_, SandboxTransferSimulateRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferSimulateRequest {
                event_type: event_type.to_owned(),
                failure_reason: None,
                test_clock_id: None,
                transfer_id: transfer_id.to_owned(),
                webhook: None,
            },
        }
    }
}
