use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_get`].

On request success, this will return a [`WatchlistScreeningEntityGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityGetRequest {
    pub entity_watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningEntityGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "entity_watchlist_screening_id" : self.params
                        .entity_watchlist_screening_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get an entity screening

Retrieve an entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityget>.*/
    pub fn watchlist_screening_entity_get(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityGetRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityGetRequest {
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
}
