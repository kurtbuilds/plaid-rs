use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_user_get`].

On request success, this will return a [`BeaconUserGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserGetRequest {
    pub beacon_user_id: String,
}
impl FluentRequest<'_, BeaconUserGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::BeaconUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "beacon_user_id" : self.params.beacon_user_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get a Beacon User

Fetch a Beacon User.

The Beacon User is returned with all of their associated information and a `status` based on the Beacon Network duplicate record and fraud checks.


See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserget>.*/
    pub fn beacon_user_get(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, BeaconUserGetRequest> {
        FluentRequest {
            client: self,
            params: BeaconUserGetRequest {
                beacon_user_id: beacon_user_id.to_owned(),
            },
        }
    }
}
