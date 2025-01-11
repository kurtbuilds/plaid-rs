use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_freddie_mac_reports_get`].

On request success, this will return a [`CreditFreddieMacReportsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacReportsGetRequest {
    pub audit_copy_token: String,
}
impl FluentRequest<'_, CreditFreddieMacReportsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditFreddieMacReportsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditFreddieMacReportsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/freddie_mac/reports/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "audit_copy_token" : self.params.audit_copy_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve an Asset Report with Freddie Mac format (aka VOA - Verification Of Assets), and a Verification Of Employment (VOE) report if this one is available. Only Freddie Mac can use this endpoint.

The `credit/asset_report/freddie_mac/get` endpoint retrieves the Verification of Assets and Verification of Employment reports.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_freddie_mac_reports_get(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, CreditFreddieMacReportsGetRequest> {
        FluentRequest {
            client: self,
            params: CreditFreddieMacReportsGetRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
}
