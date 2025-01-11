use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::dashboard_user_list`].

On request success, this will return a [`DashboardUserListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserListRequest {
    pub cursor: Option<String>,
}
impl FluentRequest<'_, DashboardUserListRequest> {
    ///Set the value of the cursor field.
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DashboardUserListRequest> {
    type Output = httpclient::InMemoryResult<crate::model::DashboardUserListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/dashboard_user/list";
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
    /**List dashboard users

The `/dashboard_user/list` endpoint provides details (such as email address) all Dashboard users associated with your account. This can use used to audit or track the list of reviewers for Monitor, Beacon, and Identity Verification products.

See endpoint docs at <https://plaid.com/docs/api/kyc-aml-users/#dashboard_userlist>.*/
    pub fn dashboard_user_list(&self) -> FluentRequest<'_, DashboardUserListRequest> {
        FluentRequest {
            client: self,
            params: DashboardUserListRequest {
                cursor: None,
            },
        }
    }
}
