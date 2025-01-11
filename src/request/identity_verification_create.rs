use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::IdentityVerificationCreateRequestUser;
/**You should use this struct via [`PlaidClient::identity_verification_create`].

On request success, this will return a [`IdentityVerificationCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationCreateRequest {
    pub client_user_id: Option<String>,
    pub gave_consent: bool,
    pub is_idempotent: Option<bool>,
    pub is_shareable: bool,
    pub template_id: String,
    pub user: Option<IdentityVerificationCreateRequestUser>,
}
impl FluentRequest<'_, IdentityVerificationCreateRequest> {
    ///Set the value of the client_user_id field.
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    ///Set the value of the is_idempotent field.
    pub fn is_idempotent(mut self, is_idempotent: bool) -> Self {
        self.params.is_idempotent = Some(is_idempotent);
        self
    }
    ///Set the value of the user field.
    pub fn user(mut self, user: IdentityVerificationCreateRequestUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IdentityVerificationCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "gave_consent" : self.params.gave_consent }));
            if let Some(ref unwrapped) = self.params.is_idempotent {
                r = r.json(serde_json::json!({ "is_idempotent" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "is_shareable" : self.params.is_shareable }));
            r = r.json(serde_json::json!({ "template_id" : self.params.template_id }));
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(serde_json::json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a new Identity Verification

Create a new Identity Verification for the user specified by the `client_user_id` field. The requirements and behavior of the verification are determined by the `template_id` provided.
If you don't know whether the associated user already has an active Identity Verification, you can specify `"is_idempotent": true` in the request body. With idempotency enabled, a new Identity Verification will only be created if one does not already exist for the associated `client_user_id` and `template_id`. If an Identity Verification is found, it will be returned unmodified with an `200 OK` HTTP status code.

You can also use this endpoint to supply information you already have collected about the user; if any of these fields are specified, the screens prompting the user to enter them will be skipped during the Link flow.


See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationcreate>.*/
    pub fn identity_verification_create(
        &self,
        gave_consent: bool,
        is_shareable: bool,
        template_id: &str,
    ) -> FluentRequest<'_, IdentityVerificationCreateRequest> {
        FluentRequest {
            client: self,
            params: IdentityVerificationCreateRequest {
                client_user_id: None,
                gave_consent,
                is_idempotent: None,
                is_shareable,
                template_id: template_id.to_owned(),
                user: None,
            },
        }
    }
}
