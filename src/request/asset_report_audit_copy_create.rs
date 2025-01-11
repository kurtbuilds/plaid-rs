use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::asset_report_audit_copy_create`].

On request success, this will return a [`AssetReportAuditCopyCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateRequest {
    pub asset_report_token: String,
    pub auditor_id: Option<String>,
}
impl FluentRequest<'_, AssetReportAuditCopyCreateRequest> {
    ///Set the value of the auditor_id field.
    pub fn auditor_id(mut self, auditor_id: &str) -> Self {
        self.params.auditor_id = Some(auditor_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AssetReportAuditCopyCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::AssetReportAuditCopyCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/audit_copy/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "asset_report_token" : self.params.asset_report_token }
                    ),
                );
            if let Some(ref unwrapped) = self.params.auditor_id {
                r = r.json(serde_json::json!({ "auditor_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create Asset Report Audit Copy

Plaid can provide an Audit Copy of any Asset Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy contains the same underlying data as the Asset Report.

To grant access to an Audit Copy, use the `/asset_report/audit_copy/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copycreate>.*/
    pub fn asset_report_audit_copy_create(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, AssetReportAuditCopyCreateRequest> {
        FluentRequest {
            client: self,
            params: AssetReportAuditCopyCreateRequest {
                asset_report_token: asset_report_token.to_owned(),
                auditor_id: None,
            },
        }
    }
}
