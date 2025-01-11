use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::transfer_questionnaire_create`].

On request success, this will return a [`TransferQuestionnaireCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferQuestionnaireCreateRequest {
    pub originator_client_id: String,
    pub redirect_uri: String,
}
impl FluentRequest<'_, TransferQuestionnaireCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferQuestionnaireCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferQuestionnaireCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/questionnaire/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            r = r.json(serde_json::json!({ "redirect_uri" : self.params.redirect_uri }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Generate a Plaid-hosted onboarding UI URL.

The `/transfer/questionnaire/create` endpoint generates a Plaid-hosted onboarding UI URL. Redirect the originator to this URL to provide their due diligence information and agree to Plaid’s terms for ACH money movement.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferquestionnairecreate>.*/
    pub fn transfer_questionnaire_create(
        &self,
        originator_client_id: &str,
        redirect_uri: &str,
    ) -> FluentRequest<'_, TransferQuestionnaireCreateRequest> {
        FluentRequest {
            client: self,
            params: TransferQuestionnaireCreateRequest {
                originator_client_id: originator_client_id.to_owned(),
                redirect_uri: redirect_uri.to_owned(),
            },
        }
    }
}
