use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{TransferLedgerSweepSimulateEventType, TransferFailure};
/**You should use this struct via [`PlaidClient::sandbox_transfer_ledger_withdraw_simulate`].

On request success, this will return a [`SandboxTransferLedgerWithdrawSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferLedgerWithdrawSimulateRequest {
    pub event_type: TransferLedgerSweepSimulateEventType,
    pub failure_reason: Option<TransferFailure>,
    pub sweep_id: String,
}
impl FluentRequest<'_, SandboxTransferLedgerWithdrawSimulateRequest> {
    ///Set the value of the failure_reason field.
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.params.failure_reason = Some(failure_reason);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferLedgerWithdrawSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferLedgerWithdrawSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/ledger/withdraw/simulate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "event_type" : self.params.event_type }));
            if let Some(ref unwrapped) = self.params.failure_reason {
                r = r.json(serde_json::json!({ "failure_reason" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "sweep_id" : self.params.sweep_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Simulate a ledger withdraw event in Sandbox

Use the `/sandbox/transfer/ledger/withdraw/simulate` endpoint to simulate a ledger withdraw event in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferledgerwithdrawsimulate>.*/
    pub fn sandbox_transfer_ledger_withdraw_simulate(
        &self,
        event_type: TransferLedgerSweepSimulateEventType,
        sweep_id: &str,
    ) -> FluentRequest<'_, SandboxTransferLedgerWithdrawSimulateRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferLedgerWithdrawSimulateRequest {
                event_type,
                failure_reason: None,
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
}
