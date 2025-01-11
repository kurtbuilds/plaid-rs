use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_review_list`].

On request success, this will return a [`WatchlistScreeningIndividualReviewListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualReviewListRequest {
    pub cursor: Option<String>,
    pub watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualReviewListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualReviewListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualReviewListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/review/list";
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
    /**List reviews for individual watchlist screenings

List all reviews for the individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualreviewlist>.*/
    pub fn watchlist_screening_individual_review_list(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualReviewListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualReviewListRequest {
                cursor: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
}
