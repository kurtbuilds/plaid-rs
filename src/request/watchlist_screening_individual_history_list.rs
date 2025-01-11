use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_history_list`].

On request success, this will return a [`WatchlistScreeningIndividualHistoryListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualHistoryListRequest {
    pub cursor: Option<String>,
    pub watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualHistoryListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualHistoryListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualHistoryListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/history/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "watchlist_screening_id" : self.params.watchlist_screening_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List history for individual watchlist screenings

List all changes to the individual watchlist screening in reverse-chronological order. If the watchlist screening has not been edited, no history will be returned.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualhistorylist>.*/
    pub fn watchlist_screening_individual_history_list(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualHistoryListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualHistoryListRequest {
                cursor: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
}
