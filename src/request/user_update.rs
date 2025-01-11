use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ConsumerReportUserIdentity;
/**You should use this struct via [`PlaidClient::user_update`].

On request success, this will return a [`UserUpdateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub consumer_report_user_identity: Option<ConsumerReportUserIdentity>,
    pub user_token: String,
}
impl FluentRequest<'_, UserUpdateRequest> {
    ///Set the value of the consumer_report_user_identity field.
    pub fn consumer_report_user_identity(
        mut self,
        consumer_report_user_identity: ConsumerReportUserIdentity,
    ) -> Self {
        self.params.consumer_report_user_identity = Some(consumer_report_user_identity);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UserUpdateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::UserUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/update";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.consumer_report_user_identity {
                r = r
                    .json(
                        serde_json::json!(
                            { "consumer_report_user_identity" : unwrapped }
                        ),
                    );
            }
            r = r.json(serde_json::json!({ "user_token" : self.params.user_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Update user information

This endpoint is used to update user information associated with an existing `user_token`. It can also be used to enable an existing `user_token` for use with Consumer Reports by Plaid Check, by adding a `consumer_report_user_identity` object to the user.

See endpoint docs at <https://plaid.com/docs/api/users/#userupdate>.*/
    pub fn user_update(&self, user_token: &str) -> FluentRequest<'_, UserUpdateRequest> {
        FluentRequest {
            client: self,
            params: UserUpdateRequest {
                consumer_report_user_identity: None,
                user_token: user_token.to_owned(),
            },
        }
    }
}
