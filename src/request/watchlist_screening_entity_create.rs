use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningEntityCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub search_terms: EntityWatchlistSearchTerms,
    pub client_user_id: Option<String>,
}
impl<'a> WatchlistScreeningEntityCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WatchlistScreeningEntityCreateResponse> {
        let mut r = self.http_client.client.post("/watchlist_screening/entity/create");
        r = r.json(json!({ "search_terms" : self.search_terms }));
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningEntityCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningEntityCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}