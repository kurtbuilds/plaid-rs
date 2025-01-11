use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_program_get`].

On request success, this will return a [`WatchlistScreeningEntityProgramGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityProgramGetRequest {
    pub entity_watchlist_program_id: String,
}
impl FluentRequest<'_, WatchlistScreeningEntityProgramGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityProgramGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityProgramGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/program/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "entity_watchlist_program_id" : self.params
                        .entity_watchlist_program_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get entity watchlist screening program

Get an entity watchlist screening program

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityprogramget>.*/
    pub fn watchlist_screening_entity_program_get(
        &self,
        entity_watchlist_program_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityProgramGetRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityProgramGetRequest {
                entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
            },
        }
    }
}
