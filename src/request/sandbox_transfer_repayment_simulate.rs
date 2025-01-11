use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_repayment_simulate`].

On request success, this will return a [`SandboxTransferRepaymentSimulateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateRequest {
    pub body: serde_json::Value,
}
impl FluentRequest<'_, SandboxTransferRepaymentSimulateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferRepaymentSimulateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferRepaymentSimulateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/repayment/simulate";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Trigger the creation of a repayment

Use the `/sandbox/transfer/repayment/simulate` endpoint to trigger the creation of a repayment. As a side effect of calling this route, a repayment is created that includes all unreimbursed returns of guaranteed transfers. If there are no such returns, an 400 error is returned.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrepaymentsimulate>.*/
    pub fn sandbox_transfer_repayment_simulate(
        &self,
        body: serde_json::Value,
    ) -> FluentRequest<'_, SandboxTransferRepaymentSimulateRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferRepaymentSimulateRequest {
                body,
            },
        }
    }
}
