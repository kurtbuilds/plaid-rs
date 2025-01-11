use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CreditBankIncomeGetRequestOptions;
/**You should use this struct via [`PlaidClient::credit_bank_income_get`].

On request success, this will return a [`CreditBankIncomeGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequest {
    pub options: Option<CreditBankIncomeGetRequestOptions>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CreditBankIncomeGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: CreditBankIncomeGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditBankIncomeGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreditBankIncomeGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/bank_income/get";
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
    /**Retrieve information from the bank accounts used for income verification

`/credit/bank_income/get` returns the bank income report(s) for a specified user. A single report corresponds to all institutions linked in a single Link session. To include multiple institutions in a single report, use [Multi-Item Link](https://plaid.com/docs/link/multi-item-link). To return older reports, use the `options.count` field.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomeget>.*/
    pub fn credit_bank_income_get(
        &self,
    ) -> FluentRequest<'_, CreditBankIncomeGetRequest> {
        FluentRequest {
            client: self,
            params: CreditBankIncomeGetRequest {
                options: None,
                user_token: None,
            },
        }
    }
}
