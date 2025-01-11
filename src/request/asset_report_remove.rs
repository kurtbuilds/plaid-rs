use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::asset_report_remove`].

On request success, this will return a [`AssetReportRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportRemoveRequest {
    pub asset_report_token: String,
}
impl FluentRequest<'_, AssetReportRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportRemoveRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/remove";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "asset_report_token" : self.params.asset_report_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Delete an Asset Report

The `/item/remove` endpoint allows you to invalidate an `access_token`, meaning you will not be able to create new Asset Reports with it. Removing an Item does not affect any Asset Reports or Audit Copies you have already created, which will remain accessible until you remove them specifically.

The `/asset_report/remove` endpoint allows you to remove access to an Asset Report. Removing an Asset Report invalidates its `asset_report_token`, meaning you will no longer be able to use it to access Report data or create new Audit Copies. Removing an Asset Report does not affect the underlying Items, but does invalidate any `audit_copy_tokens` associated with the Asset Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportremove>.*/
    pub fn asset_report_remove(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, AssetReportRemoveRequest> {
        FluentRequest {
            client: self,
            params: AssetReportRemoveRequest {
                asset_report_token: asset_report_token.to_owned(),
            },
        }
    }
}
