use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::TransferDocumentPurpose;
/**You should use this struct via [`PlaidClient::transfer_diligence_document_upload`].

On request success, this will return a [`TransferDiligenceDocumentUploadResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDiligenceDocumentUploadRequest {
    pub file: String,
    pub originator_client_id: String,
    pub purpose: TransferDocumentPurpose,
}
impl FluentRequest<'_, TransferDiligenceDocumentUploadRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferDiligenceDocumentUploadRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::TransferDiligenceDocumentUploadResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/diligence/document/upload";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "file" : self.params.file }));
            r = r
                .json(
                    serde_json::json!(
                        { "originator_client_id" : self.params.originator_client_id }
                    ),
                );
            r = r.json(serde_json::json!({ "purpose" : self.params.purpose }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Upload transfer diligence document on behalf of the originator

Third-party sender customers can use `/transfer/diligence/document/upload` endpoint to upload a document on behalf of its end customer (i.e. originator) to Plaid. You’ll need to send a request of type multipart/form-data.
You must provide the `client_id` in the `PLAID-CLIENT-ID` header and `secret` in the `PLAID-SECRET` header.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/platform-payments/#transferdiligencedocumentupload>.*/
    pub fn transfer_diligence_document_upload(
        &self,
        file: &str,
        originator_client_id: &str,
        purpose: TransferDocumentPurpose,
    ) -> FluentRequest<'_, TransferDiligenceDocumentUploadRequest> {
        FluentRequest {
            client: self,
            params: TransferDiligenceDocumentUploadRequest {
                file: file.to_owned(),
                originator_client_id: originator_client_id.to_owned(),
                purpose,
            },
        }
    }
}
