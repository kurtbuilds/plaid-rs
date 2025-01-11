use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CraLoanUnregister;
/**You should use this struct via [`PlaidClient::cra_loans_unregister`].

On request success, this will return a [`CraLoanUnregisterResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoansUnregisterRequest {
    pub loans: Vec<CraLoanUnregister>,
}
impl FluentRequest<'_, CraLoansUnregisterRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraLoansUnregisterRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraLoanUnregisterResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/loans/unregister";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "loans" : self.params.loans }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Unregister a list of loans.

`/cra/loans/unregister` indicates the loans have reached a final status and no further updates are expected.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn cra_loans_unregister(
        &self,
        loans: Vec<CraLoanUnregister>,
    ) -> FluentRequest<'_, CraLoansUnregisterRequest> {
        FluentRequest {
            client: self,
            params: CraLoansUnregisterRequest { loans },
        }
    }
}
