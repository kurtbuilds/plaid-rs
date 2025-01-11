use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::asset_report_filter`].

On request success, this will return a [`AssetReportFilterResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportFilterRequest {
    pub account_ids_to_exclude: Vec<String>,
    pub asset_report_token: String,
}
impl FluentRequest<'_, AssetReportFilterRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportFilterRequest> {
    type Output = httpclient::InMemoryResult<crate::model::AssetReportFilterResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/asset_report/filter";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "account_ids_to_exclude" : self.params.account_ids_to_exclude }
                    ),
                );
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
    /**Filter Asset Report

By default, an Asset Report will contain all of the accounts on a given Item. In some cases, you may not want the Asset Report to contain all accounts. For example, you might have the end user choose which accounts are relevant in Link using the Account Select view, which you can enable in the dashboard. Or, you might always exclude certain account types or subtypes, which you can identify by using the `/accounts/get` endpoint. To narrow an Asset Report to only a subset of accounts, use the `/asset_report/filter` endpoint.

To exclude certain Accounts from an Asset Report, first use the `/asset_report/create` endpoint to create the report, then send the `asset_report_token` along with a list of `account_ids` to exclude to the `/asset_report/filter` endpoint, to create a new Asset Report which contains only a subset of the original Asset Report's data.

Because Asset Reports are immutable, calling `/asset_report/filter` does not alter the original Asset Report in any way; rather, `/asset_report/filter` creates a new Asset Report with a new token and id. Asset Reports created via `/asset_report/filter` do not contain new Asset data, and are not billed.

Plaid will fire a [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook once generation of the filtered Asset Report has completed.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportfilter>.*/
    pub fn asset_report_filter(
        &self,
        account_ids_to_exclude: &[&str],
        asset_report_token: &str,
    ) -> FluentRequest<'_, AssetReportFilterRequest> {
        FluentRequest {
            client: self,
            params: AssetReportFilterRequest {
                account_ids_to_exclude: account_ids_to_exclude
                    .iter()
                    .map(|&x| x.to_owned())
                    .collect(),
                asset_report_token: asset_report_token.to_owned(),
            },
        }
    }
}
