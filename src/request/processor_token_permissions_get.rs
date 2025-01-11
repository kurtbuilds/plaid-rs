use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_token_permissions_get`].

On request success, this will return a [`ProcessorTokenPermissionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenPermissionsGetRequest {
    pub processor_token: String,
}
impl FluentRequest<'_, ProcessorTokenPermissionsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTokenPermissionsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorTokenPermissionsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/token/permissions/get";
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
    /**Get a processor token's product permissions

Used to get a processor token's product permissions. The `products` field will be an empty list if the processor can access all available products.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokenpermissionsget>.*/
    pub fn processor_token_permissions_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, ProcessorTokenPermissionsGetRequest> {
        FluentRequest {
            client: self,
            params: ProcessorTokenPermissionsGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
}
