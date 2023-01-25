use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WatchlistScreeningIndividualGetResponse> {
        let mut r = self.http_client.client.post("/watchlist_screening/individual/get");
        r = r.json(json!({ "watchlist_screening_id" : self.watchlist_screening_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualGetRequest<'a> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningIndividualGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}