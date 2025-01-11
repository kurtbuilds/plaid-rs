use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_monitoring_insights_subscribe`].

On request success, this will return a [`CraMonitoringInsightsSubscribeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraMonitoringInsightsSubscribeRequest {
    pub user_token: String,
    pub webhook: String,
}
impl FluentRequest<'_, CraMonitoringInsightsSubscribeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CraMonitoringInsightsSubscribeRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraMonitoringInsightsSubscribeResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/monitoring_insights/subscribe";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = r.json(serde_json::json!({ "webhook" : self.params.webhook }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Subscribe to Monitoring Insights

This endpoint allows you to subscribe to insights for a user's linked CRA items, which are updated every 14 days.

See endpoint docs at <https://plaid.com/docs/check/api/#cramonitoring_insightssubscribe>.*/
    pub fn cra_monitoring_insights_subscribe(
        &self,
        user_token: &str,
        webhook: &str,
    ) -> FluentRequest<'_, CraMonitoringInsightsSubscribeRequest> {
        FluentRequest {
            client: self,
            params: CraMonitoringInsightsSubscribeRequest {
                user_token: user_token.to_owned(),
                webhook: webhook.to_owned(),
            },
        }
    }
}
