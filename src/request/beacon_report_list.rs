use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_report_list`].

On request success, this will return a [`BeaconReportListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconReportListRequest {
    pub beacon_user_id: String,
    pub cursor: Option<String>,
}
impl FluentRequest<'_, BeaconReportListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconReportListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconReportListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/report/list";
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
    /**List Beacon Reports for a Beacon User

Use the `/beacon/report/list` endpoint to view all Beacon Reports you created for a specific Beacon User. The reports returned by this endpoint are exclusively reports you created for a specific user. A Beacon User can only have one active report at a time, but a new report can be created if a previous report has been deleted. The results from this endpoint are paginated; the `next_cursor` field will be populated if there is another page of results that can be retrieved. To fetch the next page, pass the `next_cursor` value as the `cursor` parameter in the next request.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportlist>.*/
    pub fn beacon_report_list(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, BeaconReportListRequest> {
        FluentRequest {
            client: self,
            params: BeaconReportListRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                cursor: None,
            },
        }
    }
}
