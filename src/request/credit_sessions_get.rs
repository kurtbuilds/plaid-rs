use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::credit_sessions_get`].

On request success, this will return a [`CreditSessionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditSessionsGetRequest {
    pub user_token: String,
}
impl FluentRequest<'_, CreditSessionsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditSessionsGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::CreditSessionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/credit/sessions/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Retrieve Link sessions for your user

This endpoint can be used for your end users after they complete the Link flow. This endpoint returns a list of Link sessions that your user completed, where each session includes the results from the Link flow.

These results include details about the Item that was created and some product related metadata (showing, for example, whether the user finished the bank income verification step).

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditsessionsget>.*/
    pub fn credit_sessions_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, CreditSessionsGetRequest> {
        FluentRequest {
            client: self,
            params: CreditSessionsGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
}
