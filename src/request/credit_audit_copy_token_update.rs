use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_audit_copy_token_update`].

On request success, this will return a [`CreditAuditCopyTokenUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenUpdateRequest {
    pub audit_copy_token: String,
    pub report_tokens: Vec<String>,
}
impl FluentRequest<'_, CreditAuditCopyTokenUpdateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditAuditCopyTokenUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditAuditCopyTokenUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/audit_copy_token/update";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "audit_copy_token" : self.params.audit_copy_token }
                    ),
                );
            r = r
                .json(
                    serde_json::json!({ "report_tokens" : self.params.report_tokens }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Update an Audit Copy Token

The `/credit/audit_copy_token/update` endpoint updates an existing  Audit Copy Token by adding the report tokens in the `report_tokens` field to the `audit_copy_token`. If the Audit Copy Token already contains a report of a certain type, it will be replaced with the token provided in the `report_tokens` field.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_audit_copy_token_update(
        &self,
        audit_copy_token: &str,
        report_tokens: &[&str],
    ) -> FluentRequest<'_, CreditAuditCopyTokenUpdateRequest> {
        FluentRequest {
            client: self,
            params: CreditAuditCopyTokenUpdateRequest {
                audit_copy_token: audit_copy_token.to_owned(),
                report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
}
