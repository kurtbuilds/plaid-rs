use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::IdentityDocumentsUploadsGetRequestOptions;
/**You should use this struct via [`PlaidClient::identity_documents_uploads_get`].

On request success, this will return a [`IdentityDocumentsUploadsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityDocumentsUploadsGetRequest {
    pub access_token: String,
    pub client_id: Option<String>,
    pub options: Option<IdentityDocumentsUploadsGetRequestOptions>,
    pub secret: Option<String>,
}
impl FluentRequest<'_, IdentityDocumentsUploadsGetRequest> {
    ///Set the value of the client_id field.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.params.client_id = Some(client_id.to_owned());
        self
    }
    ///Set the value of the options field.
    pub fn options(
        mut self,
        options: IdentityDocumentsUploadsGetRequestOptions,
    ) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the secret field.
    pub fn secret(mut self, secret: &str) -> Self {
        self.params.secret = Some(secret.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityDocumentsUploadsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IdentityDocumentsUploadsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity/documents/uploads/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            if let Some(ref unwrapped) = self.params.client_id {
                r = r.json(serde_json::json!({ "client_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.secret {
                r = r.json(serde_json::json!({ "secret" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Returns uploaded document identity

Use `/identity/documents/uploads/get` to retrieve identity details when using [Identity Document Upload](https://plaid.com/docs/identity/identity-document-upload/).

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identitydocumentsuploadsget>.*/
    pub fn identity_documents_uploads_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, IdentityDocumentsUploadsGetRequest> {
        FluentRequest {
            client: self,
            params: IdentityDocumentsUploadsGetRequest {
                access_token: access_token.to_owned(),
                client_id: None,
                options: None,
                secret: None,
            },
        }
    }
}
