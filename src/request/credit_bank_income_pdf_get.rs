use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_bank_income_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomePdfGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, CreditBankIncomePdfGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditBankIncomePdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/bank_income/pdf/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve information from the bank accounts used for income verification in PDF format

`/credit/bank_income/pdf/get` returns the most recent bank income report for a specified user in PDF format.  A single report corresponds to all institutions linked in a single Link session. To include multiple institutions in a single report, use [Multi-Item Link](https://plaid.com/docs/link/multi-item-link).

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomepdfget>.*/
    pub fn credit_bank_income_pdf_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditBankIncomePdfGetRequest> {
        FluentRequest {
            client: self,
            params: CreditBankIncomePdfGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
