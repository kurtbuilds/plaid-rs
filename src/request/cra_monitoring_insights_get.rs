use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::MonitoringConsumerReportPermissiblePurpose;
/**You should use this struct via [`PlaidClient::cra_monitoring_insights_get`].

On request success, this will return a [`CraMonitoringInsightsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraMonitoringInsightsGetRequest {
    pub consumer_report_permissible_purpose: MonitoringConsumerReportPermissiblePurpose,
    pub user_token: String,
}
impl FluentRequest<'_, CraMonitoringInsightsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CraMonitoringInsightsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraMonitoringInsightsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/monitoring_insights/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "consumer_report_permissible_purpose" : self.params
                        .consumer_report_permissible_purpose }
                    ),
                );
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a Monitoring Insights Report

This endpoint allows you to retrieve a Monitoring Insights report by passing in the `user_token` referred to in the webhook you received.

See endpoint docs at <https://plaid.com/docs/check/api/#cramonitoring_insightsget>.*/
    pub fn cra_monitoring_insights_get(
        &self,
        consumer_report_permissible_purpose: MonitoringConsumerReportPermissiblePurpose,
        user_token: &str,
    ) -> FluentRequest<'_, CraMonitoringInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: CraMonitoringInsightsGetRequest {
                consumer_report_permissible_purpose,
                user_token: user_token.to_owned(),
            },
        }
    }
}
