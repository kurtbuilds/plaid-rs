use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ReportType;
/**You should use this struct via [`PlaidClient::credit_relay_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayPdfGetRequest {
    pub relay_token: String,
    pub report_type: ReportType,
}
impl FluentRequest<'_, CreditRelayPdfGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayPdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/relay/pdf/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "relay_token" : self.params.relay_token }));
            r = r.json(serde_json::json!({ "report_type" : self.params.report_type }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve the pdf reports associated with a relay token that was shared with you (beta)

`/credit/relay/pdf/get` allows third parties to receive a pdf report that was shared with them, using a `relay_token` that was created by the report owner.

The `/credit/relay/pdf/get` endpoint retrieves the Asset Report in PDF format. Before calling `/credit/relay/pdf/get`, you must first create the Asset Report using `/credit/relay/create` and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

The response to `/credit/relay/pdf/get` is the PDF binary data. The `request_id` is returned in the `Plaid-Request-ID` header.

[View a sample PDF Asset Report](https://plaid.com/documents/sample-asset-report.pdf).

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelaypdfget>.*/
    pub fn credit_relay_pdf_get(
        &self,
        relay_token: &str,
        report_type: ReportType,
    ) -> FluentRequest<'_, CreditRelayPdfGetRequest> {
        FluentRequest {
            client: self,
            params: CreditRelayPdfGetRequest {
                relay_token: relay_token.to_owned(),
                report_type,
            },
        }
    }
}
