use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_report_get`].

On request success, this will return a [`BeaconReportGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportGetRequest {
    pub beacon_report_id: String,
}
impl FluentRequest<'_, BeaconReportGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconReportGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconReportGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "beacon_report_id" : self.params.beacon_report_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get a Beacon Report

Returns a Beacon report for a given Beacon report id.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportget>.*/
    pub fn beacon_report_get(
        &self,
        beacon_report_id: &str,
    ) -> FluentRequest<'_, BeaconReportGetRequest> {
        FluentRequest {
            client: self,
            params: BeaconReportGetRequest {
                beacon_report_id: beacon_report_id.to_owned(),
            },
        }
    }
}
