use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CreditBankIncomeRefreshRequestOptions;
/**You should use this struct via [`PlaidClient::credit_bank_income_refresh`].

On request success, this will return a [`CreditBankIncomeRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshRequest {
    pub options: Option<CreditBankIncomeRefreshRequestOptions>,
    pub user_token: String,
}
impl FluentRequest<'_, CreditBankIncomeRefreshRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: CreditBankIncomeRefreshRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankIncomeRefreshRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditBankIncomeRefreshResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/bank_income/refresh";
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
    /**Refresh a user's bank income information

`/credit/bank_income/refresh` refreshes the most recent bank income report data for a specific user. If the most recent bank income report is no longer valid (i.e. deleted), the endpoint will refresh the most recent valid report instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomerefresh>.*/
    pub fn credit_bank_income_refresh(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditBankIncomeRefreshRequest> {
        FluentRequest {
            client: self,
            params: CreditBankIncomeRefreshRequest {
                options: None,
                user_token: user_token.to_owned(),
            },
        }
    }
}
