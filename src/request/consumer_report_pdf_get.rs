use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::consumer_report_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerReportPdfGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, ConsumerReportPdfGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ConsumerReportPdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/consumer_report/pdf/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a PDF Reports

Retrieves all existing CRB Bank Income and Base reports for the consumer in PDF format.

Response is PDF binary data. The `request_id` is returned in the `Plaid-Request-ID` header.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn consumer_report_pdf_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, ConsumerReportPdfGetRequest> {
        FluentRequest {
            client: self,
            params: ConsumerReportPdfGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
