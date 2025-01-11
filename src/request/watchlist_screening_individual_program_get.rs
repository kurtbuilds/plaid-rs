use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_program_get`].

On request success, this will return a [`WatchlistScreeningIndividualProgramGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualProgramGetRequest {
    pub watchlist_program_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualProgramGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualProgramGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualProgramGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/program/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "watchlist_program_id" : self.params.watchlist_program_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get individual watchlist screening program

Get an individual watchlist screening program

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualprogramget>.*/
    pub fn watchlist_screening_individual_program_get(
        &self,
        watchlist_program_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualProgramGetRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualProgramGetRequest {
                watchlist_program_id: watchlist_program_id.to_owned(),
            },
        }
    }
}
