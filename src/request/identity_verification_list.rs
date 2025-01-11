use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::identity_verification_list`].

On request success, this will return a [`IdentityVerificationListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationListRequest {
    pub client_user_id: String,
    pub cursor: Option<String>,
    pub template_id: String,
}
impl FluentRequest<'_, IdentityVerificationListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IdentityVerificationListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IdentityVerificationListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/identity_verification/list";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "client_user_id" : self.params.client_user_id }),
                );
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "template_id" : self.params.template_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List Identity Verifications

Filter and list Identity Verifications created by your account

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationlist>.*/
    pub fn identity_verification_list(
        &self,
        client_user_id: &str,
        template_id: &str,
    ) -> FluentRequest<'_, IdentityVerificationListRequest> {
        FluentRequest {
            client: self,
            params: IdentityVerificationListRequest {
                client_user_id: client_user_id.to_owned(),
                cursor: None,
                template_id: template_id.to_owned(),
            },
        }
    }
}
