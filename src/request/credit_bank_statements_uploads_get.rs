use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::CreditBankStatementsUploadsGetRequestOptions;
/**You should use this struct via [`PlaidClient::credit_bank_statements_uploads_get`].

On request success, this will return a [`CreditBankStatementsUploadsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankStatementsUploadsGetRequest {
    pub options: Option<CreditBankStatementsUploadsGetRequestOptions>,
    pub user_token: String,
}
impl FluentRequest<'_, CreditBankStatementsUploadsGetRequest> {
    ///Set the value of the options field.
    pub fn options(
        mut self,
        options: CreditBankStatementsUploadsGetRequestOptions,
    ) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankStatementsUploadsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditBankStatementsUploadsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/bank_statements/uploads/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve data for a user's uploaded bank statements

`/credit/bank_statements/uploads/get` returns parsed data from bank statements uploaded by users as part of the Document Income flow. If your account is not enabled for Document Parsing, contact your account manager to request access.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_statementsuploadsget>.*/
    pub fn credit_bank_statements_uploads_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditBankStatementsUploadsGetRequest> {
        FluentRequest {
            client: self,
            params: CreditBankStatementsUploadsGetRequest {
                options: None,
                user_token: user_token.to_owned(),
            },
        }
    }
}
