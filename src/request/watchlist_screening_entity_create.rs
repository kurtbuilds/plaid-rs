use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::EntityWatchlistSearchTerms;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_create`].

On request success, this will return a [`WatchlistScreeningEntityCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityCreateRequest {
    pub client_user_id: Option<String>,
    pub search_terms: EntityWatchlistSearchTerms,
}
impl FluentRequest<'_, WatchlistScreeningEntityCreateRequest> {
    ///Set the value of the client_user_id field.
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/create";
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
    /**Create a watchlist screening for an entity

Create a new entity watchlist screening to check your customer against watchlists defined in the associated entity watchlist program. If your associated program has ongoing screening enabled, this is the profile information that will be used to monitor your customer over time.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentitycreate>.*/
    pub fn watchlist_screening_entity_create(
        &self,
        search_terms: EntityWatchlistSearchTerms,
    ) -> FluentRequest<'_, WatchlistScreeningEntityCreateRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityCreateRequest {
                client_user_id: None,
                search_terms,
            },
        }
    }
}
