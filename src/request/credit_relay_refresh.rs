use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ReportType;
/**You should use this struct via [`PlaidClient::credit_relay_refresh`].

On request success, this will return a [`CreditRelayRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayRefreshRequest {
    pub relay_token: String,
    pub report_type: ReportType,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, CreditRelayRefreshRequest> {
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayRefreshRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreditRelayRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/refresh";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "relay_token" : self.params.relay_token }));
            r = r.json(serde_json::json!({ "report_type" : self.params.report_type }));
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Refresh a report of a relay token

The `/credit/relay/refresh` endpoint allows third parties to refresh a report that was relayed to them, using a `relay_token` that was created by the report owner. A new report will be created with the original report parameters, but with the most recent data available based on the `days_requested` value of the original report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayrefresh>.*/
    pub fn credit_relay_refresh(
        &self,
        relay_token: &str,
        report_type: ReportType,
    ) -> FluentRequest<'_, CreditRelayRefreshRequest> {
        FluentRequest {
            client: self,
            params: CreditRelayRefreshRequest {
                relay_token: relay_token.to_owned(),
                report_type,
                webhook: None,
            },
        }
    }
}
