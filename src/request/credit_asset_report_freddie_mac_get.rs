use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_asset_report_freddie_mac_get`].

On request success, this will return a [`AssetReportFreddieGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAssetReportFreddieMacGetRequest {
    pub audit_copy_token: String,
}
impl FluentRequest<'_, CreditAssetReportFreddieMacGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditAssetReportFreddieMacGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::AssetReportFreddieGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/asset_report/freddie_mac/get";
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
    /**Retrieve an Asset Report with Freddie Mac format. Only Freddie Mac can use this endpoint.

The `credit/asset_report/freddie_mac/get` endpoint retrieves the Asset Report in Freddie Mac's JSON format.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_asset_report_freddie_mac_get(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, CreditAssetReportFreddieMacGetRequest> {
        FluentRequest {
            client: self,
            params: CreditAssetReportFreddieMacGetRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
}
