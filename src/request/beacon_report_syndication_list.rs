use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_report_syndication_list`].

On request success, this will return a [`BeaconReportSyndicationListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportSyndicationListRequest {
    pub beacon_user_id: String,
    pub cursor: Option<String>,
}
impl FluentRequest<'_, BeaconReportSyndicationListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, BeaconReportSyndicationListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BeaconReportSyndicationListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report_syndication/list";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "beacon_user_id" : self.params.beacon_user_id }),
                );
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List Beacon Report Syndications for a Beacon User

Use the `/beacon/report_syndication/list` endpoint to view all Beacon Reports that have been syndicated to a specific Beacon User. This endpoint returns Beacon Report Syndications which are references to Beacon Reports created either by you, or another Beacon customer, that matched the specified Beacon User. A Beacon User can have multiple active Beacon Report Syndications at once. The results from this endpoint are paginated; the `next_cursor` field will be populated if there is another page of results that can be retrieved. To fetch the next page, pass the `next_cursor` value as the `cursor` parameter in the next request.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreport_syndicationlist>.*/
    pub fn beacon_report_syndication_list(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, BeaconReportSyndicationListRequest> {
        FluentRequest {
            client: self,
            params: BeaconReportSyndicationListRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                cursor: None,
            },
        }
    }
}
