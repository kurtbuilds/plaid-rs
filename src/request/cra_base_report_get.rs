use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_base_report_get`].

On request success, this will return a [`CraBaseReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBaseReportGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, CraBaseReportGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraBaseReportGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraBaseReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/base_report/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a Base Report

This endpoint allows the customer to retrieve a Base Report. Customers should pass in the `user_token` created in `/user/create`.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn cra_base_report_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CraBaseReportGetRequest> {
        FluentRequest {
            client: self,
            params: CraBaseReportGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
