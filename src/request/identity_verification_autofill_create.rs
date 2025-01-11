use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::identity_verification_autofill_create`].

On request success, this will return a [`IdentityVerificationAutofillCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationAutofillCreateRequest {
    pub identity_verification_id: String,
}
impl FluentRequest<'_, IdentityVerificationAutofillCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationAutofillCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IdentityVerificationAutofillCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/autofill/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "identity_verification_id" : self.params
                        .identity_verification_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create autofill for an Identity Verification

Try to autofill an Identity Verification based of the provided phone number, date of birth and country of residence.

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationautofillcreate>.*/
    pub fn identity_verification_autofill_create(
        &self,
        identity_verification_id: &str,
    ) -> FluentRequest<'_, IdentityVerificationAutofillCreateRequest> {
        FluentRequest {
            client: self,
            params: IdentityVerificationAutofillCreateRequest {
                identity_verification_id: identity_verification_id.to_owned(),
            },
        }
    }
}
