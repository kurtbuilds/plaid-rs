use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::income_verification_taxforms_get`].

On request success, this will return a [`IncomeVerificationTaxformsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetRequest {
    pub access_token: Option<String>,
    pub income_verification_id: Option<String>,
}
impl FluentRequest<'_, IncomeVerificationTaxformsGetRequest> {
    ///Set the value of the access_token field.
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    ///Set the value of the income_verification_id field.
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.params.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationTaxformsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IncomeVerificationTaxformsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/income/verification/taxforms/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(serde_json::json!({ "access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.income_verification_id {
                r = r.json(serde_json::json!({ "income_verification_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Retrieve information from the tax documents used for income verification

`/income/verification/taxforms/get` returns the information collected from forms that were used to verify an end user''s income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationtaxformsget>.*/
    pub fn income_verification_taxforms_get(
        &self,
    ) -> FluentRequest<'_, IncomeVerificationTaxformsGetRequest> {
        FluentRequest {
            client: self,
            params: IncomeVerificationTaxformsGetRequest {
                access_token: None,
                income_verification_id: None,
            },
        }
    }
}
