use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    IncomeVerificationPrecheckEmployer, IncomeVerificationPrecheckPayrollInstitution,
    IncomeVerificationPrecheckMilitaryInfo, IncomeVerificationPrecheckUser,
};
/**You should use this struct via [`PlaidClient::income_verification_precheck`].

On request success, this will return a [`IncomeVerificationPrecheckResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckRequest {
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub payroll_institution: Option<IncomeVerificationPrecheckPayrollInstitution>,
    pub transactions_access_token: Option<String>,
    pub transactions_access_tokens: Option<Vec<String>>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
    pub user: Option<IncomeVerificationPrecheckUser>,
}
impl FluentRequest<'_, IncomeVerificationPrecheckRequest> {
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
    ///Set the value of the transactions_access_token field.
    pub fn transactions_access_token(mut self, transactions_access_token: &str) -> Self {
        self
            .params
            .transactions_access_token = Some(transactions_access_token.to_owned());
        self
    }
    ///Set the value of the transactions_access_tokens field.
    pub fn transactions_access_tokens(
        mut self,
        transactions_access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .transactions_access_tokens = Some(
            transactions_access_tokens
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
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
    ///Set the value of the user field.
    pub fn user(mut self, user: IncomeVerificationPrecheckUser) -> Self {
        self.params.user = Some(user);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationPrecheckRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::IncomeVerificationPrecheckResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/income/verification/precheck";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.employer {
                r = r.json(serde_json::json!({ "employer" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payroll_institution {
                r = r.json(serde_json::json!({ "payroll_institution" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.transactions_access_token {
                r = r
                    .json(
                        serde_json::json!({ "transactions_access_token" : unwrapped }),
                    );
            }
            if let Some(ref unwrapped) = self.params.transactions_access_tokens {
                r = r
                    .json(
                        serde_json::json!({ "transactions_access_tokens" : unwrapped }),
                    );
            }
            if let Some(ref unwrapped) = self.params.us_military_info {
                r = r.json(serde_json::json!({ "us_military_info" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user {
                r = r.json(serde_json::json!({ "user" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Check digital income verification eligibility and optimize conversion

`/income/verification/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification and returns a `precheck_id` that can be provided to `/link/token/create`. If the user is eligible for digital verification, providing the `precheck_id` in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the `precheck_id` can still be provided to `/link/token/create` and the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing either `employer` or `transactions_access_tokens` data will increase the chance of receiving a useful result.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/precheck` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationprecheck>.*/
    pub fn income_verification_precheck(
        &self,
    ) -> FluentRequest<'_, IncomeVerificationPrecheckRequest> {
        FluentRequest {
            client: self,
            params: IncomeVerificationPrecheckRequest {
                employer: None,
                payroll_institution: None,
                transactions_access_token: None,
                transactions_access_tokens: None,
                us_military_info: None,
                user: None,
            },
        }
    }
}
