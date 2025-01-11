use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::issues_search`].

On request success, this will return a [`IssuesSearchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesSearchRequest {
    pub item_id: Option<String>,
    pub link_session_id: Option<String>,
    pub link_session_request_id: Option<String>,
}
impl FluentRequest<'_, IssuesSearchRequest> {
    ///Set the value of the item_id field.
    pub fn item_id(mut self, item_id: &str) -> Self {
        self.params.item_id = Some(item_id.to_owned());
        self
    }
    ///Set the value of the link_session_id field.
    pub fn link_session_id(mut self, link_session_id: &str) -> Self {
        self.params.link_session_id = Some(link_session_id.to_owned());
        self
    }
    ///Set the value of the link_session_request_id field.
    pub fn link_session_request_id(mut self, link_session_request_id: &str) -> Self {
        self.params.link_session_request_id = Some(link_session_request_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IssuesSearchRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IssuesSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/issues/search";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.item_id {
                r = r.json(serde_json::json!({ "item_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.link_session_id {
                r = r.json(serde_json::json!({ "link_session_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.link_session_request_id {
                r = r.json(serde_json::json!({ "link_session_request_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Search for an Issue

Search for an issue associated with one of the following identifiers:  `item_id`, `link_session_id` or Link session `request_id`.
This endpoint returns a list of `Issue` objects, with an empty list indicating that no issues are associated with the
provided identifier. At least one of the identifiers must be provided to perform the search.

See endpoint docs at <https://plaid.com/docs/api/products/issues#issuessearch>.*/
    pub fn issues_search(&self) -> FluentRequest<'_, IssuesSearchRequest> {
        FluentRequest {
            client: self,
            params: IssuesSearchRequest {
                item_id: None,
                link_session_id: None,
                link_session_request_id: None,
            },
        }
    }
}
