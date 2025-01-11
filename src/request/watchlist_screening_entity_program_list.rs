use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_program_list`].

On request success, this will return a [`WatchlistScreeningEntityProgramListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityProgramListRequest {
    pub cursor: Option<String>,
}
impl FluentRequest<'_, WatchlistScreeningEntityProgramListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityProgramListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityProgramListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/program/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List entity watchlist screening programs

List all entity watchlist screening programs

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityprogramlist>.*/
    pub fn watchlist_screening_entity_program_list(
        &self,
    ) -> FluentRequest<'_, WatchlistScreeningEntityProgramListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityProgramListRequest {
                cursor: None,
            },
        }
    }
}
