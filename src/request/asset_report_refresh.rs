use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::AssetReportRefreshRequestOptions;
/**You should use this struct via [`PlaidClient::asset_report_refresh`].

On request success, this will return a [`AssetReportRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportRefreshRequest {
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: Option<AssetReportRefreshRequestOptions>,
}
impl FluentRequest<'_, AssetReportRefreshRequest> {
    ///Set the value of the days_requested field.
    pub fn days_requested(mut self, days_requested: i64) -> Self {
        self.params.days_requested = Some(days_requested);
        self
    }
    ///Set the value of the options field.
    pub fn options(mut self, options: AssetReportRefreshRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportRefreshRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/refresh";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "asset_report_token" : self.params.asset_report_token }
                    ),
                );
            if let Some(ref unwrapped) = self.params.days_requested {
                r = r.json(serde_json::json!({ "days_requested" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Refresh an Asset Report

An Asset Report is an immutable snapshot of a user's assets. In order to "refresh" an Asset Report you created previously, you can use the `/asset_report/refresh` endpoint to create a new Asset Report based on the old one, but with the most recent data available.

The new Asset Report will contain the same Items as the original Report, as well as the same filters applied by any call to `/asset_report/filter`. By default, the new Asset Report will also use the same parameters you submitted with your original `/asset_report/create` request, but the original `days_requested` value and the values of any parameters in the `options` object can be overridden with new values. To change these arguments, simply supply new values for them in your request to `/asset_report/refresh`. Submit an empty string ("") for any previously-populated fields you would like set as empty.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportrefresh>.*/
    pub fn asset_report_refresh(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, AssetReportRefreshRequest> {
        FluentRequest {
            client: self,
            params: AssetReportRefreshRequest {
                asset_report_token: asset_report_token.to_owned(),
                days_requested: None,
                options: None,
            },
        }
    }
}
