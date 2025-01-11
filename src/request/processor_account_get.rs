use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_account_get`].

On request success, this will return a [`ProcessorAccountGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorAccountGetRequest {
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorAccountGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorAccountGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProcessorAccountGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/account/get";
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
    /**Retrieve the account associated with a processor token

This endpoint returns the account associated with a given processor token.

This endpoint retrieves cached information, rather than extracting fresh information from the institution. As a result, the account balance returned may not be up-to-date; for realtime balance information, use `/processor/balance/get` instead. Note that some information is nullable.


See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processoraccountget>.*/
    pub fn processor_account_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorAccountGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorAccountGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
