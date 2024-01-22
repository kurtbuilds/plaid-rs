use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportPdfGetRequest {
    pub asset_report_token: String,
    pub options: Option<AssetReportPdfGetRequestOptions>,
}
impl AssetReportPdfGetRequest {}
impl FluentRequest<'_, AssetReportPdfGetRequest> {
    pub fn options(mut self, options: AssetReportPdfGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportPdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/pdf/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "asset_report_token" : self.params.asset_report_token }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}