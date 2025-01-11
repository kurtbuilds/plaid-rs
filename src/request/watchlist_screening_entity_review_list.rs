use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_review_list`].

On request success, this will return a [`WatchlistScreeningEntityReviewListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityReviewListRequest {
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningEntityReviewListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityReviewListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityReviewListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/review/list";
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
    /**List reviews for entity watchlist screenings

List all reviews for a particular entity watchlist screening. Reviews are compliance reports created by users in your organization regarding the relevance of potential hits found by Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityreviewlist>.*/
    pub fn watchlist_screening_entity_review_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityReviewListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityReviewListRequest {
                cursor: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
}
