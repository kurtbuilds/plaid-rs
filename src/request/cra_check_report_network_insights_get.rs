use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::cra_check_report_network_insights_get`].

On request success, this will return a [`CraCheckReportNetworkInsightsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportNetworkInsightsGetRequest {
    pub third_party_user_token: Option<String>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CraCheckReportNetworkInsightsGetRequest> {
    ///Set the value of the third_party_user_token field.
    pub fn third_party_user_token(mut self, third_party_user_token: &str) -> Self {
        self.params.third_party_user_token = Some(third_party_user_token.to_owned());
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CraCheckReportNetworkInsightsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraCheckReportNetworkInsightsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/check_report/network_insights/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.third_party_user_token {
                r = r.json(serde_json::json!({ "third_party_user_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.user_token {
                r = r.json(serde_json::json!({ "user_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve network attributes for the user

This endpoint allows you to retrieve the Network Insights product for your user. You should call this endpoint after you've received the `CHECK_REPORT_READY` webhook, either after the Link session for the user or after calling `/cra/check_report/create`. If the most recent consumer report for the user doesn’t have sufficient data to generate the report, or the consumer report has expired, you will receive an error indicating that you should create a new consumer report by calling `/cra/check_report/create`.

If you did not initialize Link with the `cra_network_attributes` product or have generated a report using `/cra/check_report/create`, we will generate the attributes when you call this endpoint.

See endpoint docs at <https://plaid.com/docs/check/api/#cracheck_reportnetwork_insightsget>.*/
    pub fn cra_check_report_network_insights_get(
        &self,
    ) -> FluentRequest<'_, CraCheckReportNetworkInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: CraCheckReportNetworkInsightsGetRequest {
                third_party_user_token: None,
                user_token: None,
            },
        }
    }
}
