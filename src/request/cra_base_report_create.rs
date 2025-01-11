use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ConsumerReportPermissiblePurpose;
/**You should use this struct via [`PlaidClient::cra_base_report_create`].

On request success, this will return a [`CraBaseReportCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBaseReportCreateRequest {
    pub consumer_report_permissible_purpose: ConsumerReportPermissiblePurpose,
    pub days_requested: i64,
    pub user_token: String,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, CraBaseReportCreateRequest> {
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraBaseReportCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraBaseReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/base_report/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "consumer_report_permissible_purpose" : self.params
                        .consumer_report_permissible_purpose }
                    ),
                );
            r = r
                .json(
                    serde_json::json!({ "days_requested" : self.params.days_requested }),
                );
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
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
    /**Create a Base Report

This endpoint allows the customer to create a Base Report by passing in a user token. The Base Report will be generated based on the most recently linked item from the user token.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn cra_base_report_create(
        &self,
        consumer_report_permissible_purpose: ConsumerReportPermissiblePurpose,
        days_requested: i64,
        user_token: &str,
    ) -> FluentRequest<'_, CraBaseReportCreateRequest> {
        FluentRequest {
            client: self,
            params: CraBaseReportCreateRequest {
                consumer_report_permissible_purpose,
                days_requested,
                user_token: user_token.to_owned(),
                webhook: None,
            },
        }
    }
}
