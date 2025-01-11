use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_get`].

On request success, this will return a [`WatchlistScreeningIndividualGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualGetRequest {
    pub watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/get";
            let mut r = self.client.client.post(url);
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
    /**Retrieve an individual watchlist screening

Retrieve a previously created individual watchlist screening

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualget>.*/
    pub fn watchlist_screening_individual_get(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualGetRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualGetRequest {
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
}
