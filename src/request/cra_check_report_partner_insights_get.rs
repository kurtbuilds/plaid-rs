use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CraCheckReportPartnerInsightsGetOptions;
/**You should use this struct via [`PlaidClient::cra_check_report_partner_insights_get`].

On request success, this will return a [`CraCheckReportPartnerInsightsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportPartnerInsightsGetRequest {
    pub options: Option<CraCheckReportPartnerInsightsGetOptions>,
    pub third_party_user_token: Option<String>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, CraCheckReportPartnerInsightsGetRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: CraCheckReportPartnerInsightsGetOptions) -> Self {
        self.params.options = Some(options);
        self
    }
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
for FluentRequest<'a, CraCheckReportPartnerInsightsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CraCheckReportPartnerInsightsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/check_report/partner_insights/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
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
    /**Retrieve cash flow insights from partners

This endpoint allows you to retrieve the Partner Insights report for your user. You should call this endpoint after you've received the `CHECK_REPORT_READY` webhook, either after the Link session for the user or after calling `/cra/check_report/create`. If the most recent consumer report for the user doesn’t have sufficient data to generate the base report, or the consumer report has expired, you will receive an error indicating that you should create a new consumer report by calling `/cra/check_report/create`.

If you did not initialize Link with the `credit_partner_insights` product or have generated a report using `/cra/check_report/create`, we will call our partners to generate the insights when you call this endpoint. In this case, you may optionally provide parameters under `options` to configure which insights you want to receive.

See endpoint docs at <https://plaid.com/docs/check/api/#cracheck_reportpartner_insightsget>.*/
    pub fn cra_check_report_partner_insights_get(
        &self,
    ) -> FluentRequest<'_, CraCheckReportPartnerInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: CraCheckReportPartnerInsightsGetRequest {
                options: None,
                third_party_user_token: None,
                user_token: None,
            },
        }
    }
}
