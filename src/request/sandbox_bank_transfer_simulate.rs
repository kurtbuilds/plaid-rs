use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::BankTransferFailure;
/**You should use this struct via [`PlaidClient::sandbox_bank_transfer_simulate`].

On request success, this will return a [`SandboxBankTransferSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateRequest {
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<BankTransferFailure>,
}
impl FluentRequest<'_, SandboxBankTransferSimulateRequest> {
    ///Set the value of the failure_reason field.
    pub fn failure_reason(mut self, failure_reason: BankTransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxBankTransferSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxBankTransferSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/bank_transfer/simulate";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "bank_transfer_id" : self.params.bank_transfer_id }
                    ),
                );
            r = r.json(serde_json::json!({ "event_type" : self.params.event_type }));
            if let Some(ref unwrapped) = self.params.failure_reason {
                r = r.json(serde_json::json!({ "failure_reason" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Simulate a bank transfer event in Sandbox

Use the `/sandbox/bank_transfer/simulate` endpoint to simulate a bank transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/bank_transfer/event/sync` or `/bank_transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transfersimulate>.*/
    pub fn sandbox_bank_transfer_simulate(
        &self,
        bank_transfer_id: &str,
        event_type: &str,
    ) -> FluentRequest<'_, SandboxBankTransferSimulateRequest> {
        FluentRequest {
            client: self,
            params: SandboxBankTransferSimulateRequest {
                bank_transfer_id: bank_transfer_id.to_owned(),
                event_type: event_type.to_owned(),
                failure_reason: None,
            },
        }
    }
}
