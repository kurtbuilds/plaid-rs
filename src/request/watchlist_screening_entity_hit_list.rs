use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_hit_list`].

On request success, this will return a [`WatchlistScreeningEntityHitListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityHitListRequest {
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningEntityHitListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityHitListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityHitListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/hit/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
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
    /**List hits for entity watchlist screenings

List all hits for the entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhitlist>.*/
    pub fn watchlist_screening_entity_hit_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityHitListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityHitListRequest {
                cursor: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
}
