use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ReportType;
/**You should use this struct via [`PlaidClient::credit_relay_get`].

On request success, this will return a [`AssetReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayGetRequest {
    pub include_insights: Option<bool>,
    pub relay_token: String,
    pub report_type: ReportType,
}
impl FluentRequest<'_, CreditRelayGetRequest> {
    ///Set the value of the include_insights field.
    pub fn include_insights(mut self, include_insights: bool) -> Self {
        self.params.include_insights = Some(include_insights);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.include_insights {
                r = r.json(serde_json::json!({ "include_insights" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "relay_token" : self.params.relay_token }));
            r = r.json(serde_json::json!({ "report_type" : self.params.report_type }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve the reports associated with a relay token that was shared with you

`/credit/relay/get` allows third parties to receive a report that was shared with them, using a `relay_token` that was created by the report owner.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayget>.*/
    pub fn credit_relay_get(
        &self,
        relay_token: &str,
        report_type: ReportType,
    ) -> FluentRequest<'_, CreditRelayGetRequest> {
        FluentRequest {
            client: self,
            params: CreditRelayGetRequest {
                include_insights: None,
                relay_token: relay_token.to_owned(),
                report_type,
            },
        }
    }
}
