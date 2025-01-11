use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WatchlistScreeningRequestSearchTerms;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_create`].

On request success, this will return a [`WatchlistScreeningIndividualCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualCreateRequest {
    pub client_user_id: Option<String>,
    pub search_terms: WatchlistScreeningRequestSearchTerms,
}
impl FluentRequest<'_, WatchlistScreeningIndividualCreateRequest> {
    ///Set the value of the client_user_id field.
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "search_terms" : self.params.search_terms }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a watchlist screening for a person

Create a new Watchlist Screening to check your customer against watchlists defined in the associated Watchlist Program. If your associated program has ongoing screening enabled, this is the profile information that will be used to monitor your customer over time.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualcreate>.*/
    pub fn watchlist_screening_individual_create(
        &self,
        search_terms: WatchlistScreeningRequestSearchTerms,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualCreateRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualCreateRequest {
                client_user_id: None,
                search_terms,
            },
        }
    }
}
