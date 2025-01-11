use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::BeaconUserStatus;
/**You should use this struct via [`PlaidClient::beacon_user_review`].

On request success, this will return a [`BeaconUserGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserReviewRequest {
    pub beacon_user_id: String,
    pub status: BeaconUserStatus,
}
impl FluentRequest<'_, BeaconUserReviewRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserReviewRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/review";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "beacon_user_id" : self.params.beacon_user_id }),
                );
            r = r.json(serde_json::json!({ "status" : self.params.status }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Review a Beacon User

Update the status of a Beacon User.

When updating a Beacon User's status via this endpoint, Plaid validates that the status change is consistent with the related state for this Beacon User. Specifically, we will check:

1. Whether there are any associated Beacon Reports connected to the Beacon User, and
2. Whether there are any confirmed Beacon Report Syndications connected to the Beacon User.

When updating a Beacon User's status to "rejected", we enforce that either a Beacon Report has been created for the Beacon User or a Beacon Report Syndication has been confirmed.
When updating a Beacon User's status to "cleared", we enforce that there are no active Beacon Reports or confirmed Beacon Report Syndications associated with the user. If you previously created a Beacon Report for this user, you must delete it before updating the Beacon User's status to "cleared".
There are no restrictions on updating a Beacon User's status to "pending_review".

If these conditions are not met, the request will be rejected with an error explaining the issue.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserreview>.*/
    pub fn beacon_user_review(
        &self,
        beacon_user_id: &str,
        status: BeaconUserStatus,
    ) -> FluentRequest<'_, BeaconUserReviewRequest> {
        FluentRequest {
            client: self,
            params: BeaconUserReviewRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                status,
            },
        }
    }
}
