use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::dashboard_user_get`].

On request success, this will return a [`DashboardUserGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserGetRequest {
    pub dashboard_user_id: String,
}
impl FluentRequest<'_, DashboardUserGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DashboardUserGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::DashboardUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/dashboard_user/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "dashboard_user_id" : self.params.dashboard_user_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve a dashboard user

The `/dashboard_user/get` endpoint provides details (such as email address) about a specific Dashboard user based on the `dashboard_user_id` field, which is returned in the `audit_trail` object of certain Monitor and Beacon endpoints. This can be used to identify the specific reviewer who performed a Dashboard action.

See endpoint docs at <https://plaid.com/docs/api/kyc-aml-users/#dashboard_userget>.*/
    pub fn dashboard_user_get(
        &self,
        dashboard_user_id: &str,
    ) -> FluentRequest<'_, DashboardUserGetRequest> {
        FluentRequest {
            client: self,
            params: DashboardUserGetRequest {
                dashboard_user_id: dashboard_user_id.to_owned(),
            },
        }
    }
}
