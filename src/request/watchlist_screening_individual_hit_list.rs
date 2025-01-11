use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_hit_list`].

On request success, this will return a [`WatchlistScreeningIndividualHitListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualHitListRequest {
    pub cursor: Option<String>,
    pub watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualHitListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualHitListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualHitListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/hit/list";
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
    /**List hits for individual watchlist screening

List all hits found by Plaid for a particular individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualhitlist>.*/
    pub fn watchlist_screening_individual_hit_list(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualHitListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualHitListRequest {
                cursor: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
}
