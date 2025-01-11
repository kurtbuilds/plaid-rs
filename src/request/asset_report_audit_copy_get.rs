use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::asset_report_audit_copy_get`].

On request success, this will return a [`AssetReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportAuditCopyGetRequest {
    pub audit_copy_token: String,
}
impl FluentRequest<'_, AssetReportAuditCopyGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AssetReportAuditCopyGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/audit_copy/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "audit_copy_token" : self.params.audit_copy_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve an Asset Report Audit Copy

`/asset_report/audit_copy/get` allows auditors to get a copy of an Asset Report that was previously shared via the `/asset_report/audit_copy/create` endpoint.  The caller of `/asset_report/audit_copy/create` must provide the `audit_copy_token` to the auditor.  This token can then be used to call `/asset_report/audit_copy/create`.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_audit_copy_get(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, AssetReportAuditCopyGetRequest> {
        FluentRequest {
            client: self,
            params: AssetReportAuditCopyGetRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
}
