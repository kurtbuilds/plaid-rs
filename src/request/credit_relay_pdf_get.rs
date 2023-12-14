use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_relay_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRelayPdfGetRequest {
    pub relay_token: String,
    pub report_type: String,
}
impl CreditRelayPdfGetRequest {}
impl FluentRequest<'_, CreditRelayPdfGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditRelayPdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/relay/pdf/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}