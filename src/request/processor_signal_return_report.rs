use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::processor_signal_return_report`].

On request success, this will return a [`ProcessorSignalReturnReportResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalReturnReportRequest {
    pub client_transaction_id: String,
    pub processor_token: String,
    pub return_code: String,
    pub returned_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl FluentRequest<'_, ProcessorSignalReturnReportRequest> {
    ///Set the value of the returned_at field.
    pub fn returned_at(mut self, returned_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.returned_at = Some(returned_at);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalReturnReportRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::ProcessorSignalReturnReportResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/processor/signal/return/report";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "client_transaction_id" : self.params.client_transaction_id }
                    ),
                );
            r = r
                .json(
                    serde_json::json!(
                        { "processor_token" : self.params.processor_token }
                    ),
                );
            r = r.json(serde_json::json!({ "return_code" : self.params.return_code }));
            if let Some(ref unwrapped) = self.params.returned_at {
                r = r.json(serde_json::json!({ "returned_at" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Report a return for an ACH transaction

Call the `/processor/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/processor/signal/evaluate` endpoint. Your feedback will be used by the model to incorporate the latest risk trend in your portfolio.

If you are using the [Plaid Transfer product](https://www.plaid.com/docs/transfer) to create transfers, it is not necessary to use this endpoint, as Plaid already knows whether the transfer was returned.

See endpoint docs at <https://plaid.com/docs/api/processor-partners/#processorsignalreturnreport>.*/
    pub fn processor_signal_return_report(
        &self,
        client_transaction_id: &str,
        processor_token: &str,
        return_code: &str,
    ) -> FluentRequest<'_, ProcessorSignalReturnReportRequest> {
        FluentRequest {
            client: self,
            params: ProcessorSignalReturnReportRequest {
                client_transaction_id: client_transaction_id.to_owned(),
                processor_token: processor_token.to_owned(),
                return_code: return_code.to_owned(),
                returned_at: None,
            },
        }
    }
}
