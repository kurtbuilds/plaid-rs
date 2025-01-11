use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CraLoanApplication;
/**You should use this struct via [`PlaidClient::cra_loans_applications_register`].

On request success, this will return a [`CraLoansApplicationsRegisterResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoansApplicationsRegisterRequest {
    pub applications: Vec<CraLoanApplication>,
}
impl FluentRequest<'_, CraLoansApplicationsRegisterRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CraLoansApplicationsRegisterRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraLoansApplicationsRegisterResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/loans/applications/register";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "applications" : self.params.applications }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Register loan applications and decisions.

`/cra/loans/applications/register` registers loan applications and decisions.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn cra_loans_applications_register(
        &self,
        applications: Vec<CraLoanApplication>,
    ) -> FluentRequest<'_, CraLoansApplicationsRegisterRequest> {
        FluentRequest {
            client: self,
            params: CraLoansApplicationsRegisterRequest {
                applications,
            },
        }
    }
}
