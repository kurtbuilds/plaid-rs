use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::AssetReportPDFGetRequestOptions;
/**You should use this struct via [`PlaidClient::asset_report_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportPdfGetRequest {
    pub asset_report_token: String,
    pub options: Option<AssetReportPdfGetRequestOptions>,
}
impl FluentRequest<'_, AssetReportPdfGetRequest> {
    ///Set the value of the options field.
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
            r = r
                .json(
                    serde_json::json!(
                        { "asset_report_token" : self.params.asset_report_token }
                    ),
                );
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
    /**Retrieve a PDF Asset Report

The `/asset_report/pdf/get` endpoint retrieves the Asset Report in PDF format. Before calling `/asset_report/pdf/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

The response to `/asset_report/pdf/get` is the PDF binary data. The `request_id`  is returned in the `Plaid-Request-ID` header.

[View a sample PDF Asset Report](https://plaid.com/documents/sample-asset-report.pdf).

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportpdfget>.*/
    pub fn asset_report_pdf_get(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, AssetReportPdfGetRequest> {
        FluentRequest {
            client: self,
            params: AssetReportPdfGetRequest {
                asset_report_token: asset_report_token.to_owned(),
                options: None,
            },
        }
    }
}
