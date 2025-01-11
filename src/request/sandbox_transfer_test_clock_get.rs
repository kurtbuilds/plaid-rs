use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_get`].

On request success, this will return a [`SandboxTransferTestClockGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockGetRequest {
    pub test_clock_id: String,
}
impl FluentRequest<'_, SandboxTransferTestClockGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferTestClockGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/test_clock/get";
            let mut r = self.client.client.post(url);
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
    /**Get a test clock

Use the `/sandbox/transfer/test_clock/get` endpoint to get a `test_clock` in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockget>.*/
    pub fn sandbox_transfer_test_clock_get(
        &self,
        test_clock_id: &str,
    ) -> FluentRequest<'_, SandboxTransferTestClockGetRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferTestClockGetRequest {
                test_clock_id: test_clock_id.to_owned(),
            },
        }
    }
}
