use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::WatchlistScreeningStatus;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_list`].

On request success, this will return a [`WatchlistScreeningIndividualListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualListRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub status: Option<WatchlistScreeningStatus>,
    pub watchlist_program_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualListRequest> {
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
for FluentRequest<'a, WatchlistScreeningIndividualListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/list";
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
            if let Some(ref unwrapped) = self.params.status {
                r = r.json(serde_json::json!({ "status" : unwrapped }));
            }
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
    /**List Individual Watchlist Screenings

List previously created watchlist screenings for individuals

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividuallist>.*/
    pub fn watchlist_screening_individual_list(
        &self,
        watchlist_program_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualListRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualListRequest {
                assignee: None,
                client_user_id: None,
                cursor: None,
                status: None,
                watchlist_program_id: watchlist_program_id.to_owned(),
            },
        }
    }
}
