use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_bank_employment_get`].

On request success, this will return a [`CreditBankEmploymentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankEmploymentGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, CreditBankEmploymentGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankEmploymentGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditBankEmploymentGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/credit/v1/bank_employment/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve information from the bank accounts used for employment verification

`/credit/bank_employment/get` returns the employment report(s) derived from bank transaction data for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_employmentget>.*/
    pub fn credit_bank_employment_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditBankEmploymentGetRequest> {
        FluentRequest {
            client: self,
            params: CreditBankEmploymentGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
