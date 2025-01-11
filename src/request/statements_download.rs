use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::statements_download`].

On request success, this will return a [`String`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementsDownloadRequest {
    pub access_token: String,
    pub statement_id: String,
}
impl FluentRequest<'_, StatementsDownloadRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, StatementsDownloadRequest> {
    type Output = httpclient::InMemoryResult<String>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/statements/download";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
            r = r.json(serde_json::json!({ "statement_id" : self.params.statement_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a single statement.

The `/statements/download` endpoint retrieves a single statement PDF in binary format.  The response will contain a `Plaid-Content-Hash` header containing a SHA 256 checksum of the statement. This can be used to verify that the file being sent by Plaid is the same file that was downloaded to your system.

See endpoint docs at <https://plaid.com/docs/api/products/statements#statementsdownload>.*/
    pub fn statements_download(
        &self,
        access_token: &str,
        statement_id: &str,
    ) -> FluentRequest<'_, StatementsDownloadRequest> {
        FluentRequest {
            client: self,
            params: StatementsDownloadRequest {
                access_token: access_token.to_owned(),
                statement_id: statement_id.to_owned(),
            },
        }
    }
}
