use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::IncomeVerificationCreateRequestOptions;
/**You should use this struct via [`PlaidClient::income_verification_create`].

On request success, this will return a [`IncomeVerificationCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequest {
    pub options: Option<IncomeVerificationCreateRequestOptions>,
    pub precheck_id: Option<String>,
    pub webhook: String,
}
impl FluentRequest<'_, IncomeVerificationCreateRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: IncomeVerificationCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the precheck_id field.
    pub fn precheck_id(mut self, precheck_id: &str) -> Self {
        self.params.precheck_id = Some(precheck_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IncomeVerificationCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/income/verification/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.precheck_id {
                r = r.json(serde_json::json!({ "precheck_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "webhook" : self.params.webhook }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Create an income verification instance

`/income/verification/create` begins the income verification process by returning an `income_verification_id`. You can then provide the `income_verification_id` to `/link/token/create` under the `income_verification` parameter in order to create a Link instance that will prompt the user to go through the income verification flow. Plaid will fire an `INCOME` webhook once the user completes the Payroll Income flow, or when the uploaded documents in the Document Income flow have finished processing.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationcreate>.*/
    pub fn income_verification_create(
        &self,
        webhook: &str,
    ) -> FluentRequest<'_, IncomeVerificationCreateRequest> {
        FluentRequest {
            client: self,
            params: IncomeVerificationCreateRequest {
                options: None,
                precheck_id: None,
                webhook: webhook.to_owned(),
            },
        }
    }
}
