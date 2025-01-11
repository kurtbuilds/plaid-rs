use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::sandbox_transfer_test_clock_list`].

On request success, this will return a [`SandboxTransferTestClockListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockListRequest {
    pub count: Option<i64>,
    pub end_virtual_time: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<i64>,
    pub start_virtual_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, SandboxTransferTestClockListRequest> {
    ///Set the value of the count field.
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    ///Set the value of the end_virtual_time field.
    pub fn end_virtual_time(
        mut self,
        end_virtual_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.end_virtual_time = Some(end_virtual_time);
        self
    }
    ///Set the value of the offset field.
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
    ///Set the value of the start_virtual_time field.
    pub fn start_virtual_time(
        mut self,
        start_virtual_time: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.start_virtual_time = Some(start_virtual_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxTransferTestClockListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxTransferTestClockListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/transfer/test_clock/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(serde_json::json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.end_virtual_time {
                r = r.json(serde_json::json!({ "end_virtual_time" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(serde_json::json!({ "offset" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.start_virtual_time {
                r = r.json(serde_json::json!({ "start_virtual_time" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List test clocks

Use the `/sandbox/transfer/test_clock/list` endpoint to see a list of all your test clocks in the Sandbox environment, by ascending `virtual_time`. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired test clocks.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clocklist>.*/
    pub fn sandbox_transfer_test_clock_list(
        &self,
    ) -> FluentRequest<'_, SandboxTransferTestClockListRequest> {
        FluentRequest {
            client: self,
            params: SandboxTransferTestClockListRequest {
                count: None,
                end_virtual_time: None,
                offset: None,
                start_virtual_time: None,
            },
        }
    }
}
