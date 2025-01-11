use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CraLoanUpdate;
/**You should use this struct via [`PlaidClient::cra_loans_update`].

On request success, this will return a [`CraLoansUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoansUpdateRequest {
    pub loans: Vec<CraLoanUpdate>,
}
impl FluentRequest<'_, CraLoansUpdateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraLoansUpdateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraLoansUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/loans/update";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "loans" : self.params.loans }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Updates loan data.

`/cra/loans/update` updates loan information such as the status and payment history.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn cra_loans_update(
        &self,
        loans: Vec<CraLoanUpdate>,
    ) -> FluentRequest<'_, CraLoansUpdateRequest> {
        FluentRequest {
            client: self,
            params: CraLoansUpdateRequest { loans },
        }
    }
}
