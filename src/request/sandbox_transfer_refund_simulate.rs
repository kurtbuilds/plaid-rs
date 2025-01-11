use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferFailure;
/**You should use this struct via [`PlaidClient::sandbox_transfer_refund_simulate`].

On request success, this will return a [`SandboxTransferRefundSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferRefundSimulateRequest {
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
    pub refund_id: String,
    pub test_clock_id: Option<String>,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, SandboxTransferRefundSimulateRequest> {
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
for FluentRequest<'a, SandboxTransferRefundSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferRefundSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/refund/simulate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "event_type" : self.params.event_type }));
            if let Some(ref unwrapped) = self.params.failure_reason {
                r = r.json(serde_json::json!({ "failure_reason" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "refund_id" : self.params.refund_id }));
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
    /**Simulate a refund event in Sandbox

Use the `/sandbox/transfer/refund/simulate` endpoint to simulate a refund event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrefundsimulate>.*/
    pub fn sandbox_transfer_refund_simulate(
        &self,
        event_type: &str,
        refund_id: &str,
    ) -> FluentRequest<'_, SandboxTransferRefundSimulateRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferRefundSimulateRequest {
                event_type: event_type.to_owned(),
                failure_reason: None,
                refund_id: refund_id.to_owned(),
                test_clock_id: None,
                webhook: None,
            },
        }
    }
}
