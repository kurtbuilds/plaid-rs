use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_identity_get`].

On request success, this will return a [`ProcessorIdentityGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityGetRequest {
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorIdentityGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorIdentityGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProcessorIdentityGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/identity/get";
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
    /**Retrieve Identity data

The `/processor/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processoridentityget>.*/
    pub fn processor_identity_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorIdentityGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorIdentityGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
