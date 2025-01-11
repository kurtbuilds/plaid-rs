use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CreditPayrollIncomeRefreshRequestOptions;
/**You should use this struct via [`PlaidClient::credit_payroll_income_refresh`].

On request success, this will return a [`CreditPayrollIncomeRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshRequest {
    pub options: Option<CreditPayrollIncomeRefreshRequestOptions>,
    pub user_token: String,
}
impl FluentRequest<'_, CreditPayrollIncomeRefreshRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: CreditPayrollIncomeRefreshRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomeRefreshRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditPayrollIncomeRefreshResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/payroll_income/refresh";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Refresh a digital payroll income verification

`/credit/payroll_income/refresh` refreshes a given digital payroll income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomerefresh>.*/
    pub fn credit_payroll_income_refresh(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditPayrollIncomeRefreshRequest> {
        FluentRequest {
            client: self,
            params: CreditPayrollIncomeRefreshRequest {
                options: None,
                user_token: user_token.to_owned(),
            },
        }
    }
}
