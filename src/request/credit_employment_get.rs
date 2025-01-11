use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_employment_get`].

On request success, this will return a [`CreditEmploymentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditEmploymentGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, CreditEmploymentGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditEmploymentGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreditEmploymentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/employment/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a summary of an individual's employment information

`/credit/employment/get` returns a list of items with employment information from a user's payroll provider that was verified by an end user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditemploymentget>.*/
    pub fn credit_employment_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditEmploymentGetRequest> {
        FluentRequest {
            client: self,
            params: CreditEmploymentGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
