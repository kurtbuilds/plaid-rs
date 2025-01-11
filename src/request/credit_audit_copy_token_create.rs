use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_audit_copy_token_create`].

On request success, this will return a [`CreditAuditCopyTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateRequest {
    pub report_tokens: Vec<String>,
}
impl FluentRequest<'_, CreditAuditCopyTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditAuditCopyTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::CreditAuditCopyTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/audit_copy_token/create";
            let mut r = self.client.client.post(url);
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
    /**Create Asset or Income Report Audit Copy Token

Plaid can create an Audit Copy token of an Asset Report and/or Income Report to share with participating Government Sponsored Entity (GSE). If you participate in the Day 1 Certainty™ program, Plaid can supply an Audit Copy token directly to Fannie Mae on your behalf. An Audit Copy token contains the same underlying data as the Asset Report and/or Income Report (result of /credit/payroll_income/get).

Use the `/credit/audit_copy_token/create` endpoint to create an `audit_copy_token` and then pass that token to the GSE who needs access.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditaudit_copy_tokencreate>.*/
    pub fn credit_audit_copy_token_create(
        &self,
        report_tokens: &[&str],
    ) -> FluentRequest<'_, CreditAuditCopyTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: CreditAuditCopyTokenCreateRequest {
                report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
}
