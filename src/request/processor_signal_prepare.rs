use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_signal_prepare`].

On request success, this will return a [`ProcessorSignalPrepareResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalPrepareRequest {
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorSignalPrepareRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorSignalPrepareRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorSignalPrepareResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/signal/prepare";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Opt-in a processor token to Signal

When a processor token is not initialized with Signal, call `/processor/signal/prepare` to opt-in that processor token to the Signal data collection process, which will improve the accuracy of the Signal score.

If this endpoint is called with a processor token that is already initialized with Signal, it will return a 200 response and will not modify the processor token.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorsignalprepare>.*/
    pub fn processor_signal_prepare(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorSignalPrepareRequest> {
        FluentRequest {
            client: self,
            params: ProcessorSignalPrepareRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
