use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::income_verification_documents_download`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeVerificationDocumentsDownloadRequest {
    pub access_token: Option<String>,
    pub document_id: Option<String>,
    pub income_verification_id: Option<String>,
}
impl FluentRequest<'_, IncomeVerificationDocumentsDownloadRequest> {
    ///Set the value of the access_token field.
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.params.access_token = Some(access_token.to_owned());
        self
    }
    ///Set the value of the document_id field.
    pub fn document_id(mut self, document_id: &str) -> Self {
        self.params.document_id = Some(document_id.to_owned());
        self
    }
    ///Set the value of the income_verification_id field.
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.params.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, IncomeVerificationDocumentsDownloadRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/income/verification/documents/download";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.access_token {
                r = r.json(serde_json::json!({ "access_token" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.document_id {
                r = r.json(serde_json::json!({ "document_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.income_verification_id {
                r = r.json(serde_json::json!({ "income_verification_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**(Deprecated) Download the original documents used for income verification

`/income/verification/documents/download` provides the ability to download the source documents associated with the verification.

If Document Income was used, the documents will be those the user provided in Link. For Payroll Income, the most recent files available
for download from the payroll provider will be available from this endpoint.

The response to `/income/verification/documents/download` is a ZIP file in binary data. If a `document_id` is passed, a single document will be contained in this file.
If not, the response will contain all documents associated with the verification.

The `request_id` is returned in the `Plaid-Request-ID` header.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationdocumentsdownload>.*/
    pub fn income_verification_documents_download(
        &self,
    ) -> FluentRequest<'_, IncomeVerificationDocumentsDownloadRequest> {
        FluentRequest {
            client: self,
            params: IncomeVerificationDocumentsDownloadRequest {
                access_token: None,
                document_id: None,
                income_verification_id: None,
            },
        }
    }
}
