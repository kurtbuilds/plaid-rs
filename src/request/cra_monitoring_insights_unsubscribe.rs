use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_monitoring_insights_unsubscribe`].

On request success, this will return a [`CraMonitoringInsightsUnsubscribeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraMonitoringInsightsUnsubscribeRequest {
    pub subscription_id: String,
}
impl FluentRequest<'_, CraMonitoringInsightsUnsubscribeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CraMonitoringInsightsUnsubscribeRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraMonitoringInsightsUnsubscribeResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/monitoring_insights/unsubscribe";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "subscription_id" : self.params.subscription_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Unsubscribe from Monitoring Insights

This endpoint allows you to unsubscribe from previously subscribed Monitoring Insights.

See endpoint docs at <https://plaid.com/docs/check/api/#cramonitoring_insightsunsubscribe>.*/
    pub fn cra_monitoring_insights_unsubscribe(
        &self,
        subscription_id: &str,
    ) -> FluentRequest<'_, CraMonitoringInsightsUnsubscribeRequest> {
        FluentRequest {
            client: self,
            params: CraMonitoringInsightsUnsubscribeRequest {
                subscription_id: subscription_id.to_owned(),
            },
        }
    }
}
