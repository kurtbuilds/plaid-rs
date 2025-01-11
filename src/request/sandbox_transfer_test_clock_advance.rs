use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_advance`].

On request success, this will return a [`SandboxTransferTestClockAdvanceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockAdvanceRequest {
    pub new_virtual_time: chrono::DateTime<chrono::Utc>,
    pub test_clock_id: String,
}
impl FluentRequest<'_, SandboxTransferTestClockAdvanceRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockAdvanceRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferTestClockAdvanceResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/test_clock/advance";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "new_virtual_time" : self.params.new_virtual_time }
                    ),
                );
            r = r
                .json(
                    serde_json::json!({ "test_clock_id" : self.params.test_clock_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Advance a test clock

Use the `/sandbox/transfer/test_clock/advance` endpoint to advance a `test_clock` in the Sandbox environment.

A test clock object represents an independent timeline and has a `virtual_time` field indicating the current timestamp of the timeline. A test clock can be advanced by incrementing `virtual_time`, but may never go back to a lower `virtual_time`.

If a test clock is advanced, we will simulate the changes that ought to occur during the time that elapsed.

For example, a client creates a weekly recurring transfer with a test clock set at t. When the client advances the test clock by setting `virtual_time` = t + 15 days, 2 new originations should be created, along with the webhook events.

The advancement of the test clock from its current `virtual_time` should be limited such that there are no more than 20 originations resulting from the advance operation on each `recurring_transfer` associated with the `test_clock`.

For example, if the recurring transfer associated with this test clock originates once every 4 weeks, you can advance the `virtual_time` up to 80 weeks on each API call.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockadvance>.*/
    pub fn sandbox_transfer_test_clock_advance(
        &self,
        new_virtual_time: chrono::DateTime<chrono::Utc>,
        test_clock_id: &str,
    ) -> FluentRequest<'_, SandboxTransferTestClockAdvanceRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferTestClockAdvanceRequest {
                new_virtual_time,
                test_clock_id: test_clock_id.to_owned(),
            },
        }
    }
}
