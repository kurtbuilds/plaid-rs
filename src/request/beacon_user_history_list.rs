use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::beacon_user_history_list`].

On request success, this will return a [`BeaconUserHistoryListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserHistoryListRequest {
    pub beacon_user_id: String,
    pub cursor: Option<String>,
}
impl FluentRequest<'_, BeaconUserHistoryListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BeaconUserHistoryListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::BeaconUserHistoryListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beacon/user/history/list";
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
    /**List a Beacon User's history

List all changes to the Beacon User in reverse-chronological order.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserhistorylist>.*/
    pub fn beacon_user_history_list(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, BeaconUserHistoryListRequest> {
        FluentRequest {
            client: self,
            params: BeaconUserHistoryListRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                cursor: None,
            },
        }
    }
}
