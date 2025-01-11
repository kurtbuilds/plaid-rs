use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ConsumerReportPermissiblePurpose;
/**You should use this struct via [`PlaidClient::cra_check_report_create`].

On request success, this will return a [`CraCheckReportCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraCheckReportCreateRequest {
    pub consumer_report_permissible_purpose: ConsumerReportPermissiblePurpose,
    pub days_requested: i64,
    pub days_required: Option<i64>,
    pub user_token: String,
    pub webhook: String,
}
pub struct CraCheckReportCreateRequired<'a> {
    pub consumer_report_permissible_purpose: ConsumerReportPermissiblePurpose,
    pub days_requested: i64,
    pub user_token: &'a str,
    pub webhook: &'a str,
}
impl FluentRequest<'_, CraCheckReportCreateRequest> {
    ///Set the value of the days_required field.
    pub fn days_required(mut self, days_required: i64) -> Self {
        self.params.days_required = Some(days_required);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CraCheckReportCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CraCheckReportCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/cra/check_report/create";
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
            if let Some(ref unwrapped) = self.params.days_required {
                r = r.json(serde_json::json!({ "days_required" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = r.json(serde_json::json!({ "webhook" : self.params.webhook }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a Consumer Report

`/cra/check_report/create` creates a Consumer Report powered by Plaid Check. You can call this endpoint to create a new report if `consumer_report_permissible_purpose` was omitted during Link token creation. If you did provide a `consumer_report_permissible_purpose` during Link token creation, then Plaid Check will automatically begin creating a Consumer Report once the user completes the Link process, and it is not necessary to call `/cra/check_report/create` before retrieving the report.

 `/cra/check_report/create` can also be used to refresh data in an existing report. A Consumer Report will last for 24 hours before expiring; you should call any `/get` endpoints on the report before it expires. If a report expires, you can call `/cra/check_report/create` again to re-generate it. Note that refreshing or regenerating a report is a billable event.

See endpoint docs at <https://plaid.com/docs/check/api/#cracheck_reportcreate>.*/
    pub fn cra_check_report_create(
        &self,
        args: CraCheckReportCreateRequired,
    ) -> FluentRequest<'_, CraCheckReportCreateRequest> {
        FluentRequest {
            client: self,
            params: CraCheckReportCreateRequest {
                consumer_report_permissible_purpose: args
                    .consumer_report_permissible_purpose,
                days_requested: args.days_requested,
                days_required: None,
                user_token: args.user_token.to_owned(),
                webhook: args.webhook.to_owned(),
            },
        }
    }
}
