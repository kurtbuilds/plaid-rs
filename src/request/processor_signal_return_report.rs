use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSignalReturnReportRequest {
    pub client_transaction_id: String,
    pub processor_token: String,
    pub return_code: String,
    pub returned_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl ProcessorSignalReturnReportRequest {}
impl FluentRequest<'_, ProcessorSignalReturnReportRequest> {
    pub fn returned_at(mut self, returned_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.returned_at = Some(returned_at);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorSignalReturnReportRequest> {
    type Output = httpclient::InMemoryResult<ProcessorSignalReturnReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/signal/return/report";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}