use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::asset_report_audit_copy_remove`].

On request success, this will return a [`AssetReportAuditCopyRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveRequest {
    pub audit_copy_token: String,
}
impl FluentRequest<'_, AssetReportAuditCopyRemoveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AssetReportAuditCopyRemoveRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::AssetReportAuditCopyRemoveResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/audit_copy/remove";
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
    /**Remove Asset Report Audit Copy

The `/asset_report/audit_copy/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Asset Report, the Asset Report itself and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copyremove>.*/
    pub fn asset_report_audit_copy_remove(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, AssetReportAuditCopyRemoveRequest> {
        FluentRequest {
            client: self,
            params: AssetReportAuditCopyRemoveRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
}
