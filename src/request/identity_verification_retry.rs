use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    IdentityVerificationRetryRequestStepsObject, Strategy,
    IdentityVerificationRequestUser,
};
/**You should use this struct via [`PlaidClient::identity_verification_retry`].

On request success, this will return a [`IdentityVerificationRetryResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationRetryRequest {
    pub client_user_id: String,
    pub is_shareable: Option<bool>,
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
    pub strategy: Strategy,
    pub template_id: String,
    pub user: Option<IdentityVerificationRequestUser>,
}
impl FluentRequest<'_, IdentityVerificationRetryRequest> {
    ///Set the value of the is_shareable field.
    pub fn is_shareable(mut self, is_shareable: bool) -> Self {
        self.params.is_shareable = Some(is_shareable);
        self
    }
    ///Set the value of the steps field.
    pub fn steps(mut self, steps: IdentityVerificationRetryRequestStepsObject) -> Self {
        self.params.steps = Some(steps);
        self
    }
    ///Set the value of the user field.
    pub fn user(mut self, user: IdentityVerificationRequestUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationRetryRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IdentityVerificationRetryResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/retry";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "client_user_id" : self.params.client_user_id }),
                );
            if let Some(ref unwrapped) = self.params.is_shareable {
                r = r.json(serde_json::json!({ "is_shareable" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.steps {
                r = r.json(serde_json::json!({ "steps" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "strategy" : self.params.strategy }));
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
    /**Retry an Identity Verification

Allow a customer to retry their Identity Verification

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationretry>.*/
    pub fn identity_verification_retry(
        &self,
        client_user_id: &str,
        strategy: Strategy,
        template_id: &str,
    ) -> FluentRequest<'_, IdentityVerificationRetryRequest> {
        FluentRequest {
            client: self,
            params: IdentityVerificationRetryRequest {
                client_user_id: client_user_id.to_owned(),
                is_shareable: None,
                steps: None,
                strategy,
                template_id: template_id.to_owned(),
                user: None,
            },
        }
    }
}
