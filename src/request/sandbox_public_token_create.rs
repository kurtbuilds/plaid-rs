use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{Products, SandboxPublicTokenCreateRequestOptions};
/**You should use this struct via [`PlaidClient::sandbox_public_token_create`].

On request success, this will return a [`SandboxPublicTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequest {
    pub initial_products: Vec<Products>,
    pub institution_id: String,
    pub options: Option<SandboxPublicTokenCreateRequestOptions>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, SandboxPublicTokenCreateRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: SandboxPublicTokenCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxPublicTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::SandboxPublicTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/sandbox/public_token/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "initial_products" : self.params.initial_products }
                    ),
                );
            r = r
                .json(
                    serde_json::json!({ "institution_id" : self.params.institution_id }),
                );
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(serde_json::json!({ "user_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a test Item

Use the `/sandbox/public_token/create` endpoint to create a valid `public_token`  for an arbitrary institution ID, initial products, and test credentials. The created `public_token` maps to a new Sandbox Item. You can then call `/item/public_token/exchange` to exchange the `public_token` for an `access_token` and perform all API actions. `/sandbox/public_token/create` can also be used with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data, or with Plaid's [pre-populated Sandbox test accounts](https://plaid.com/docs/sandbox/test-credentials/).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate>.*/
    pub fn sandbox_public_token_create(
        &self,
        initial_products: Vec<Products>,
        institution_id: &str,
    ) -> FluentRequest<'_, SandboxPublicTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: SandboxPublicTokenCreateRequest {
                initial_products,
                institution_id: institution_id.to_owned(),
                options: None,
                user_token: None,
            },
        }
    }
}
