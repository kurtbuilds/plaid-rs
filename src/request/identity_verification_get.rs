use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::identity_verification_get`].

On request success, this will return a [`IdentityVerificationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationGetRequest {
    pub identity_verification_id: String,
}
impl FluentRequest<'_, IdentityVerificationGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IdentityVerificationGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/get";
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
    /**Retrieve Identity Verification

Retrieve a previously created Identity Verification.

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationget>.*/
    pub fn identity_verification_get(
        &self,
        identity_verification_id: &str,
    ) -> FluentRequest<'_, IdentityVerificationGetRequest> {
        FluentRequest {
            client: self,
            params: IdentityVerificationGetRequest {
                identity_verification_id: identity_verification_id.to_owned(),
            },
        }
    }
}
