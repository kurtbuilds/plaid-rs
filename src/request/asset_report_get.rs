use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::AssetReportGetRequestOptions;
/**You should use this struct via [`PlaidClient::asset_report_get`].

On request success, this will return a [`AssetReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportGetRequest {
    pub asset_report_token: Option<String>,
    pub fast_report: Option<bool>,
    pub include_insights: Option<bool>,
    pub options: Option<AssetReportGetRequestOptions>,
    pub user_token: Option<String>,
}
impl FluentRequest<'_, AssetReportGetRequest> {
    ///Set the value of the asset_report_token field.
    pub fn asset_report_token(mut self, asset_report_token: &str) -> Self {
        self.params.asset_report_token = Some(asset_report_token.to_owned());
        self
    }
    ///Set the value of the fast_report field.
    pub fn fast_report(mut self, fast_report: bool) -> Self {
        self.params.fast_report = Some(fast_report);
        self
    }
    ///Set the value of the include_insights field.
    pub fn include_insights(mut self, include_insights: bool) -> Self {
        self.params.include_insights = Some(include_insights);
        self
    }
    ///Set the value of the options field.
    pub fn options(mut self, options: AssetReportGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the user_token field.
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.params.user_token = Some(user_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.asset_report_token {
                r = r.json(serde_json::json!({ "asset_report_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.fast_report {
                r = r.json(serde_json::json!({ "fast_report" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.include_insights {
                r = r.json(serde_json::json!({ "include_insights" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
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
    /**Retrieve an Asset Report

The `/asset_report/get` endpoint retrieves the Asset Report in JSON format. Before calling `/asset_report/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

By default, an Asset Report includes transaction descriptions as returned by the bank, as opposed to parsed and categorized by Plaid. You can also receive cleaned and categorized transactions, as well as additional insights like merchant name or location information. We call this an Asset Report with Insights. An Asset Report with Insights provides transaction category, location, and merchant information in addition to the transaction strings provided in a standard Asset Report. To retrieve an Asset Report with Insights, call `/asset_report/get` endpoint with `include_insights` set to `true`.

For latency-sensitive applications, you can optionally call `/asset_report/create` with `options.add_ons` set to `["fast_assets"]`. This will cause Plaid to create two versions of the Asset Report: one with only current and available balance and identity information, and then later on the complete Asset Report. You will receive separate webhooks for each version of the Asset Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportget>.*/
    pub fn asset_report_get(&self) -> FluentRequest<'_, AssetReportGetRequest> {
        FluentRequest {
            client: self,
            params: AssetReportGetRequest {
                asset_report_token: None,
                fast_report: None,
                include_insights: None,
                options: None,
                user_token: None,
            },
        }
    }
}
