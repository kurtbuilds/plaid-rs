use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_create`].

On request success, this will return a [`SandboxTransferTestClockCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockCreateRequest {
    pub virtual_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, SandboxTransferTestClockCreateRequest> {
    ///Set the value of the virtual_time field.
    pub fn virtual_time(mut self, virtual_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.virtual_time = Some(virtual_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferTestClockCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/test_clock/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.virtual_time {
                r = r.json(serde_json::json!({ "virtual_time" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a test clock

Use the `/sandbox/transfer/test_clock/create` endpoint to create a `test_clock` in the Sandbox environment.

A test clock object represents an independent timeline and has a `virtual_time` field indicating the current timestamp of the timeline. Test clocks are used for testing recurring transfers in Sandbox.

A test clock can be associated with up to 5 recurring transfers.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockcreate>.*/
    pub fn sandbox_transfer_test_clock_create(
        &self,
    ) -> FluentRequest<'_, SandboxTransferTestClockCreateRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferTestClockCreateRequest {
                virtual_time: None,
            },
        }
    }
}
