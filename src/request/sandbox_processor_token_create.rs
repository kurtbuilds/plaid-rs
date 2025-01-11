use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::SandboxProcessorTokenCreateRequestOptions;
/**You should use this struct via [`PlaidClient::sandbox_processor_token_create`].

On request success, this will return a [`SandboxProcessorTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequest {
    pub institution_id: String,
    pub options: Option<SandboxProcessorTokenCreateRequestOptions>,
}
impl FluentRequest<'_, SandboxProcessorTokenCreateRequest> {
    ///Set the value of the options field.
    pub fn options(
        mut self,
        options: SandboxProcessorTokenCreateRequestOptions,
    ) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxProcessorTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxProcessorTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/processor_token/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "institution_id" : self.params.institution_id }),
                );
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a test Item and processor token

Use the `/sandbox/processor_token/create` endpoint to create a valid `processor_token` for an arbitrary institution ID and test credentials. The created `processor_token` corresponds to a new Sandbox Item. You can then use this `processor_token` with the `/processor/` API endpoints in Sandbox. You can also use `/sandbox/processor_token/create` with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxprocessor_tokencreate>.*/
    pub fn sandbox_processor_token_create(
        &self,
        institution_id: &str,
    ) -> FluentRequest<'_, SandboxProcessorTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: SandboxProcessorTokenCreateRequest {
                institution_id: institution_id.to_owned(),
                options: None,
            },
        }
    }
}
