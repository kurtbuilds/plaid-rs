use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferPlatformRequirementSubmission;
/**You should use this struct via [`PlaidClient::transfer_platform_requirement_submit`].

On request success, this will return a [`TransferPlatformRequirementSubmitResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPlatformRequirementSubmitRequest {
    pub originator_client_id: String,
    pub requirement_submissions: Vec<TransferPlatformRequirementSubmission>,
}
impl FluentRequest<'_, TransferPlatformRequirementSubmitRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferPlatformRequirementSubmitRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferPlatformRequirementSubmitResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/platform/requirement/submit";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "requirement_submissions" : self.params.requirement_submissions
                        }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Submit onboarding requirements for Scaled Platform originators

The `/transfer/platform/requirement/submit` endpoint allows platforms to submit onboarding requirements for an originator as part of the Scaled Platform Transfer offering.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferplatformrequirementsubmit>.*/
    pub fn transfer_platform_requirement_submit(
        &self,
        originator_client_id: &str,
        requirement_submissions: Vec<TransferPlatformRequirementSubmission>,
    ) -> FluentRequest<'_, TransferPlatformRequirementSubmitRequest> {
        FluentRequest {
            client: self,
            params: TransferPlatformRequirementSubmitRequest {
                originator_client_id: originator_client_id.to_owned(),
                requirement_submissions,
            },
        }
    }
}
