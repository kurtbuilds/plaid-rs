use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    WatchlistScreeningIndividualUpdateRequestResettableFieldList,
    UpdateIndividualScreeningRequestSearchTerms, WatchlistScreeningStatus,
};
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_update`].

On request success, this will return a [`WatchlistScreeningIndividualUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualUpdateRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub reset_fields: Option<
        WatchlistScreeningIndividualUpdateRequestResettableFieldList,
    >,
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    pub status: Option<WatchlistScreeningStatus>,
    pub watchlist_screening_id: String,
}
impl FluentRequest<'_, WatchlistScreeningIndividualUpdateRequest> {
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
        reset_fields: WatchlistScreeningIndividualUpdateRequestResettableFieldList,
    ) -> Self {
        self.params.reset_fields = Some(reset_fields);
        self
    }
    ///Set the value of the search_terms field.
    pub fn search_terms(
        mut self,
        search_terms: UpdateIndividualScreeningRequestSearchTerms,
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
for FluentRequest<'a, WatchlistScreeningIndividualUpdateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::WatchlistScreeningIndividualUpdateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/individual/update";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.assignee {
                r = r.json(serde_json::json!({ "assignee" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(serde_json::json!({ "client_user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.reset_fields {
                r = r.json(serde_json::json!({ "reset_fields" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.search_terms {
                r = r.json(serde_json::json!({ "search_terms" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.status {
                r = r.json(serde_json::json!({ "status" : unwrapped }));
            }
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
    /**Update individual watchlist screening

Update a specific individual watchlist screening. This endpoint can be used to add additional customer information, correct outdated information, add a reference id, assign the individual to a reviewer, and update which program it is associated with. Please note that you may not update `search_terms` and `status` at the same time since editing `search_terms` may trigger an automatic `status` change.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualupdate>.*/
    pub fn watchlist_screening_individual_update(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, WatchlistScreeningIndividualUpdateRequest> {
        FluentRequest {
            client: self,
            params: WatchlistScreeningIndividualUpdateRequest {
                assignee: None,
                client_user_id: None,
                reset_fields: None,
                search_terms: None,
                status: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
}
