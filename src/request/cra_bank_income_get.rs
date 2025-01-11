use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_bank_income_get`].

On request success, this will return a [`CraBankIncomeGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBankIncomeGetRequest {
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CraBankIncomeGetRequest> {
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraBankIncomeGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraBankIncomeGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/bank_income/get";
            let mut r = self.client.client.post(url);
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

`/cra/bank_income/get` returns the bank income report(s) for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#crabank_incomeget>.*/
    pub fn cra_bank_income_get(&self) -> FluentRequest<'_, CraBankIncomeGetRequest> {
        FluentRequest {
            client: self,
            params: CraBankIncomeGetRequest {
                user_token: None,
            },
        }
    }
}
