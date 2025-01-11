use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_auth_get`].

On request success, this will return a [`ProcessorAuthGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorAuthGetRequest {
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorAuthGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorAuthGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::ProcessorAuthGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/auth/get";
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
    /**Retrieve Auth data

The `/processor/auth/get` endpoint returns the bank account and bank identification number (such as the routing number, for US accounts), for a checking or savings account that''s associated with a given `processor_token`. The endpoint also returns high-level account data and balances when available.

Versioning note: API versions 2019-05-29 and earlier use a different schema for the `numbers` object returned by this endpoint. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2020-09-14).


See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorauthget>.*/
    pub fn processor_auth_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorAuthGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorAuthGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
