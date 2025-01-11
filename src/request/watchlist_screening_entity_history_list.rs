use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_history_list`].

On request success, this will return a [`WatchlistScreeningEntityHistoryListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityHistoryListRequest {
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningEntityHistoryListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityHistoryListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityHistoryListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/history/list";
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
    /**List history for entity watchlist screenings

List all changes to the entity watchlist screening in reverse-chronological order. If the watchlist screening has not been edited, no history will be returned.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhistorylist>.*/
    pub fn watchlist_screening_entity_history_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityHistoryListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityHistoryListRequest {
                cursor: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
}
