use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CraLoanRegister;
/**You should use this struct via [`PlaidClient::cra_loans_register`].

On request success, this will return a [`CraLoansRegisterResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoansRegisterRequest {
    pub loans: Vec<CraLoanRegister>,
}
impl FluentRequest<'_, CraLoansRegisterRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraLoansRegisterRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraLoansRegisterResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/loans/register";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "loans" : self.params.loans }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Register a list of loans to their applicants.

`/cra/loans/register` registers a list of loans to their applicants.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn cra_loans_register(
        &self,
        loans: Vec<CraLoanRegister>,
    ) -> FluentRequest<'_, CraLoansRegisterRequest> {
        FluentRequest {
            client: self,
            params: CraLoansRegisterRequest { loans },
        }
    }
}
