use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::network_insights_report_get`].

On request success, this will return a [`NetworkInsightsReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInsightsReportGetRequest {
    pub access_tokens: Vec<String>,
}
impl FluentRequest<'_, NetworkInsightsReportGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, NetworkInsightsReportGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::NetworkInsightsReportGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/network_insights/report/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "access_tokens" : self.params.access_tokens }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve network insights for the provided `access_tokens`

This endpoint allows you to retrieve the Network Insights from a list of `access_tokens`.

See endpoint docs at <https://plaid.com/docs/api/network_insights/report/#get>.*/
    pub fn network_insights_report_get(
        &self,
        access_tokens: &[&str],
    ) -> FluentRequest<'_, NetworkInsightsReportGetRequest> {
        FluentRequest {
            client: self,
            params: NetworkInsightsReportGetRequest {
                access_tokens: access_tokens.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
}
