use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ProcessorBalanceGetRequestOptions;
/**You should use this struct via [`PlaidClient::processor_balance_get`].

On request success, this will return a [`ProcessorBalanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequest {
    pub options: Option<ProcessorBalanceGetRequestOptions>,
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorBalanceGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: ProcessorBalanceGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProcessorBalanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/balance/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
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
    /**Retrieve Balance data

The `/processor/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/processor/balance/get` forces the available and current balance fields to be refreshed rather than cached.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorbalanceget>.*/
    pub fn processor_balance_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorBalanceGetRequest {
                options: None,
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
