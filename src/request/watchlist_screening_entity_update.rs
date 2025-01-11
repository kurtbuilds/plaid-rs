use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    WatchlistScreeningEntityUpdateRequestResettableFieldList,
    UpdateEntityScreeningRequestSearchTerms, WatchlistScreeningStatus,
};
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_update`].

On request success, this will return a [`WatchlistScreeningEntityUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityUpdateRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub entity_watchlist_screening_id: String,
    pub reset_fields: Option<WatchlistScreeningEntityUpdateRequestResettableFieldList>,
    pub search_terms: Option<UpdateEntityScreeningRequestSearchTerms>,
    pub status: Option<WatchlistScreeningStatus>,
}
impl FluentRequest<'_, WatchlistScreeningEntityUpdateRequest> {
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
    ///Set the value of the reset_fields field.
    pub fn reset_fields(
        mut self,
        reset_fields: WatchlistScreeningEntityUpdateRequestResettableFieldList,
    ) -> Self {
        self.params.reset_fields = Some(reset_fields);
        self
    }
    ///Set the value of the search_terms field.
    pub fn search_terms(
        mut self,
        search_terms: UpdateEntityScreeningRequestSearchTerms,
    ) -> Self {
        self.params.search_terms = Some(search_terms);
        self
    }
    ///Set the value of the status field.
    pub fn status(mut self, status: WatchlistScreeningStatus) -> Self {
        self.params.status = Some(status);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningEntityUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/update";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.assignee {
                r = r.json(serde_json::json!({ "assignee" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            r = r
                .json(
                    serde_json::json!(
                        { "entity_watchlist_screening_id" : self.params
                        .entity_watchlist_screening_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.reset_fields {
                r = r.json(serde_json::json!({ "reset_fields" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.search_terms {
                r = r.json(serde_json::json!({ "search_terms" : unwrapped }));
            }
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
    /**Update an entity screening

Update an entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityupdate>.*/
    pub fn watchlist_screening_entity_update(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningEntityUpdateRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningEntityUpdateRequest {
                assignee: None,
                client_user_id: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
                reset_fields: None,
                search_terms: None,
                status: None,
            },
        }
    }
}
