use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::LinkProfileEligibilityCheckUser;
/**You should use this struct via [`PlaidClient::link_profile_eligibility_check`].

On request success, this will return a [`LinkProfileEligibilityCheckResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkProfileEligibilityCheckRequest {
    pub link_session_id: String,
    pub user: LinkProfileEligibilityCheckUser,
}
impl FluentRequest<'_, LinkProfileEligibilityCheckRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, LinkProfileEligibilityCheckRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::LinkProfileEligibilityCheckResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/link/profile/eligibility/check";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "link_session_id" : self.params.link_session_id }
                    ),
                );
            r = r.json(serde_json::json!({ "user" : self.params.user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Check profile eligibility

The `/link/profile/eligibility/check` endpoint can be used to check whether a user with the
supplied phone number has a saved profile that satisfies customer-defined eligibility
requirements.

See endpoint docs at <https://plaid.com/docs/api/link/#profileeligibilitycheck>.*/
    pub fn link_profile_eligibility_check(
        &self,
        link_session_id: &str,
        user: LinkProfileEligibilityCheckUser,
    ) -> FluentRequest<'_, LinkProfileEligibilityCheckRequest> {
        FluentRequest {
            client: self,
            params: LinkProfileEligibilityCheckRequest {
                link_session_id: link_session_id.to_owned(),
                user,
            },
        }
    }
}
