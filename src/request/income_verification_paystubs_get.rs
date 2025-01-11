use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::income_verification_paystubs_get`].

On request success, this will return a [`IncomeVerificationPaystubsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetRequest {
    pub access_token: Option<String>,
    pub income_verification_id: Option<String>,
}
impl FluentRequest<'_, IncomeVerificationPaystubsGetRequest> {
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
for FluentRequest<'a, IncomeVerificationPaystubsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IncomeVerificationPaystubsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/income/verification/paystubs/get";
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
    /**(Deprecated) Retrieve information from the paystubs used for income verification

`/income/verification/paystubs/get` returns the information collected from the paystubs that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationpaystubsget>.*/
    pub fn income_verification_paystubs_get(
        &self,
    ) -> FluentRequest<'_, IncomeVerificationPaystubsGetRequest> {
        FluentRequest {
            client: self,
            params: IncomeVerificationPaystubsGetRequest {
                access_token: None,
                income_verification_id: None,
            },
        }
    }
}
