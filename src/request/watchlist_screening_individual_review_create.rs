use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_review_create`].

On request success, this will return a [`WatchlistScreeningIndividualReviewCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualReviewCreateRequest {
    pub comment: Option<String>,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualReviewCreateRequest> {
    ///Set the value of the comment field.
    pub fn comment(mut self, comment: &str) -> Self {
        self.params.comment = Some(comment.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualReviewCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualReviewCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/review/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.comment {
                r = r.json(serde_json::json!({ "comment" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!({ "confirmed_hits" : self.params.confirmed_hits }),
                );
            r = r
                .json(
                    serde_json::json!({ "dismissed_hits" : self.params.dismissed_hits }),
                );
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
    /**Create a review for an individual watchlist screening

Create a review for the individual watchlist screening. Reviews are compliance reports created by users in your organization regarding the relevance of potential hits found by Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualreviewcreate>.*/
    pub fn watchlist_screening_individual_review_create(
        &self,
        confirmed_hits: &[&str],
        dismissed_hits: &[&str],
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualReviewCreateRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualReviewCreateRequest {
                comment: None,
                confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
                dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
}
