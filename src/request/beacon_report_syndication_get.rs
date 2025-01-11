use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_report_syndication_get`].

On request success, this will return a [`BeaconReportSyndicationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportSyndicationGetRequest {
    pub beacon_report_syndication_id: String,
}
impl FluentRequest<'_, BeaconReportSyndicationGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, BeaconReportSyndicationGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BeaconReportSyndicationGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report_syndication/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "beacon_report_syndication_id" : self.params
                        .beacon_report_syndication_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get a Beacon Report Syndication

Returns a Beacon Report Syndication for a given Beacon Report Syndication id.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreport_syndicationget>.*/
    pub fn beacon_report_syndication_get(
        &self,
        beacon_report_syndication_id: &str,
    ) -> FluentRequest<'_, BeaconReportSyndicationGetRequest> {
        FluentRequest {
            client: self,
            params: BeaconReportSyndicationGetRequest {
                beacon_report_syndication_id: beacon_report_syndication_id.to_owned(),
            },
        }
    }
}
