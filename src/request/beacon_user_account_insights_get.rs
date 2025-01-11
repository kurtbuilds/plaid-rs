use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_user_account_insights_get`].

On request success, this will return a [`BeaconUserAccountInsightsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserAccountInsightsGetRequest {
    pub access_token: String,
    pub beacon_user_id: String,
}
impl FluentRequest<'_, BeaconUserAccountInsightsGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, BeaconUserAccountInsightsGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BeaconUserAccountInsightsGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/account_insights/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "access_token" : self.params.access_token }));
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
    /**Get Account Insights for a Beacon User

Get Account Insights for all Accounts linked to this Beacon User. The insights for each account are computed based on the information that was last retrieved from the financial institution.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuseraccount_insightsget>.*/
    pub fn beacon_user_account_insights_get(
        &self,
        access_token: &str,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, BeaconUserAccountInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: BeaconUserAccountInsightsGetRequest {
                access_token: access_token.to_owned(),
                beacon_user_id: beacon_user_id.to_owned(),
            },
        }
    }
}
