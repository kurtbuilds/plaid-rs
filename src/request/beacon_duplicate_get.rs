use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_duplicate_get`].

On request success, this will return a [`BeaconDuplicateGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconDuplicateGetRequest {
    pub beacon_duplicate_id: String,
}
impl FluentRequest<'_, BeaconDuplicateGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconDuplicateGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconDuplicateGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/duplicate/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "beacon_duplicate_id" : self.params.beacon_duplicate_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get a Beacon Duplicate

Returns a Beacon Duplicate for a given Beacon Duplicate id.

A Beacon Duplicate represents a pair of similar Beacon Users within your organization.

Two Beacon User revisions are returned for each Duplicate record in either the `beacon_user1` or `beacon_user2` response fields.

The `analysis` field in the response indicates which fields matched between `beacon_user1` and `beacon_user2`.


See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconduplicateget>.*/
    pub fn beacon_duplicate_get(
        &self,
        beacon_duplicate_id: &str,
    ) -> FluentRequest<'_, BeaconDuplicateGetRequest> {
        FluentRequest {
            client: self,
            params: BeaconDuplicateGetRequest {
                beacon_duplicate_id: beacon_duplicate_id.to_owned(),
            },
        }
    }
}
