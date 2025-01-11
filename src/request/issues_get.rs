use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::issues_get`].

On request success, this will return a [`IssuesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesGetRequest {
    pub issue_id: String,
}
impl FluentRequest<'_, IssuesGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IssuesGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IssuesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/issues/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "issue_id" : self.params.issue_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get an Issue

Retrieve detailed information about a specific `Issue`. This endpoint returns a single `Issue` object.

See endpoint docs at <https://plaid.com/docs/api/products/issues/#issuesget>.*/
    pub fn issues_get(&self, issue_id: &str) -> FluentRequest<'_, IssuesGetRequest> {
        FluentRequest {
            client: self,
            params: IssuesGetRequest {
                issue_id: issue_id.to_owned(),
            },
        }
    }
}
