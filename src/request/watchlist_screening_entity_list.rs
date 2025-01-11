use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WatchlistScreeningStatus;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_list`].

On request success, this will return a [`WatchlistScreeningEntityListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityListRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub entity_watchlist_program_id: String,
    pub status: Option<WatchlistScreeningStatus>,
}
impl FluentRequest<'_, WatchlistScreeningEntityListRequest> {
    ///Set the value of the assignee field.
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.params.assignee = Some(assignee.to_owned());
        self
    }
    ///Set the value of the client_user_id field.
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    ///Set the value of the status field.
    pub fn status(mut self, status: WatchlistScreeningStatus) -> Self {
        self.params.status = Some(status);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.assignee {
                r = r.json(serde_json::json!({ "assignee" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(serde_json::json!({ "cursor" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "entity_watchlist_program_id" : self.params
                        .entity_watchlist_program_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.status {
                r = r.json(serde_json::json!({ "status" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List entity watchlist screenings

List all entity screenings.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentitylist>.*/
    pub fn watchlist_screening_entity_list(
        &self,
        entity_watchlist_program_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityListRequest {
                assignee: None,
                client_user_id: None,
                cursor: None,
                entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
                status: None,
            },
        }
    }
}
