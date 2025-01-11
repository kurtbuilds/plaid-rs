use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::issues_subscribe`].

On request success, this will return a [`IssuesSubscribeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuesSubscribeRequest {
    pub issue_id: String,
    pub webhook: Option<String>,
}
impl FluentRequest<'_, IssuesSubscribeRequest> {
    ///Set the value of the webhook field.
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.params.webhook = Some(webhook.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, IssuesSubscribeRequest> {
    type Output = httpclient::InMemoryResult<crate::model::IssuesSubscribeResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/issues/subscribe";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "issue_id" : self.params.issue_id }));
            if let Some(ref unwrapped) = self.params.webhook {
                r = r.json(serde_json::json!({ "webhook" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Subscribe to an Issue

Allows a user to subscribe to updates on a specific `Issue` using a POST method. Subscribers will receive webhook notifications when the issue status changes, particularly when resolved.

See endpoint docs at <https://plaid.com/docs/api/products/issues/#issuessubscribe>.*/
    pub fn issues_subscribe(
        &self,
        issue_id: &str,
    ) -> FluentRequest<'_, IssuesSubscribeRequest> {
        FluentRequest {
            client: self,
            params: IssuesSubscribeRequest {
                issue_id: issue_id.to_owned(),
                webhook: None,
            },
        }
    }
}
