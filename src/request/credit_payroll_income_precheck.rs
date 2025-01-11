use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    IncomeVerificationPrecheckEmployer, IncomeVerificationPrecheckPayrollInstitution,
    IncomeVerificationPrecheckMilitaryInfo,
};
/**You should use this struct via [`PlaidClient::credit_payroll_income_precheck`].

On request success, this will return a [`CreditPayrollIncomePrecheckResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckRequest {
    pub access_tokens: Option<Vec<String>>,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CreditPayrollIncomePrecheckRequest> {
    ///Set the value of the access_tokens field.
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    ///Set the value of the employer field.
    pub fn employer(mut self, employer: IncomeVerificationPrecheckEmployer) -> Self {
        self.params.employer = Some(employer);
        self
    }
    ///Set the value of the payroll_institution field.
    pub fn payroll_institution(
        mut self,
        payroll_institution: IncomeVerificationPrecheckPayrollInstitution,
    ) -> Self {
        self.params.payroll_institution = Some(payroll_institution);
        self
    }
    ///Set the value of the us_military_info field.
    pub fn us_military_info(
        mut self,
        us_military_info: IncomeVerificationPrecheckMilitaryInfo,
    ) -> Self {
        self.params.us_military_info = Some(us_military_info);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditPayrollIncomePrecheckRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditPayrollIncomePrecheckResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/payroll_income/precheck";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_tokens {
                r = r.json(serde_json::json!({ "access_tokens" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.employer {
                r = r.json(serde_json::json!({ "employer" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payroll_institution {
                r = r.json(serde_json::json!({ "payroll_institution" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.us_military_info {
                r = r.json(serde_json::json!({ "us_military_info" : unwrapped }));
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
    /**Check income verification eligibility and optimize conversion

`/credit/payroll_income/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification. If the user is eligible for digital verification, that information will be associated with the user token, and in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing `employer` data will increase the chance of receiving a useful result.

When testing in Sandbox, you can control the results by providing special test values in the `employer` and `access_tokens` fields. `employer_good` and `employer_bad` will result in `HIGH` and `LOW` confidence values, respectively. `employer_multi` will result in a `HIGH` confidence with multiple payroll options. Likewise, `access_good` and `access_bad` will result in `HIGH` and `LOW` confidence values, respectively. Any other value for `employer` and `access_tokens` in Sandbox will result in `UNKNOWN` confidence.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeprecheck>.*/
    pub fn credit_payroll_income_precheck(
        &self,
    ) -> FluentRequest<'_, CreditPayrollIncomePrecheckRequest> {
        FluentRequest {
            client: self,
            params: CreditPayrollIncomePrecheckRequest {
                access_tokens: None,
                employer: None,
                payroll_institution: None,
                us_military_info: None,
                user_token: None,
            },
        }
    }
}
