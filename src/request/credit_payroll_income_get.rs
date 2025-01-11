use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CreditPayrollIncomeGetRequestOptions;
/**You should use this struct via [`PlaidClient::credit_payroll_income_get`].

On request success, this will return a [`CreditPayrollIncomeGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomeGetRequest {
    pub options: Option<CreditPayrollIncomeGetRequestOptions>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CreditPayrollIncomeGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: CreditPayrollIncomeGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditPayrollIncomeGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditPayrollIncomeGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/payroll_income/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(serde_json::json!({ "user_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a user's payroll information

This endpoint gets payroll income information for a specific user, either as a result of the user connecting to their payroll provider or uploading a pay related document.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeget>.*/
    pub fn credit_payroll_income_get(
        &self,
    ) -> FluentRequest<'_, CreditPayrollIncomeGetRequest> {
        FluentRequest {
            client: self,
            params: CreditPayrollIncomeGetRequest {
                options: None,
                user_token: None,
            },
        }
    }
}
