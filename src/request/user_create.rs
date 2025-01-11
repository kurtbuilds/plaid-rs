use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::ConsumerReportUserIdentity;
/**You should use this struct via [`PlaidClient::user_create`].

On request success, this will return a [`UserCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub client_user_id: String,
    pub consumer_report_user_identity: Option<ConsumerReportUserIdentity>,
    pub end_customer: Option<String>,
}
impl FluentRequest<'_, UserCreateRequest> {
    ///Set the value of the consumer_report_user_identity field.
    pub fn consumer_report_user_identity(
        mut self,
        consumer_report_user_identity: ConsumerReportUserIdentity,
    ) -> Self {
        self.params.consumer_report_user_identity = Some(consumer_report_user_identity);
        self
    }
    ///Set the value of the end_customer field.
    pub fn end_customer(mut self, end_customer: &str) -> Self {
        self.params.end_customer = Some(end_customer.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UserCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::UserCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/user/create";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!({ "client_user_id" : self.params.client_user_id }),
                );
            if let Some(ref unwrapped) = self.params.consumer_report_user_identity {
                r = r
                    .json(
                        serde_json::json!(
                            { "consumer_report_user_identity" : unwrapped }
                        ),
                    );
            }
            if let Some(ref unwrapped) = self.params.end_customer {
                r = r.json(serde_json::json!({ "end_customer" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create user

This endpoint should be called for each of your end users before they begin a Plaid Check or Income flow, or a Multi-Item Link flow. This provides you a single token to access all data associated with the user. You should only create one per end user.

The `consumer_report_user_identity` object must be present in order to create a Plaid Check Consumer Report for a user. If it is not provided during the `/user/create` call, it can be added later by calling `/user/update`.

If you call the endpoint multiple times with the same `client_user_id`, the first creation call will succeed and the rest will fail with an error message indicating that the user has been created for the given `client_user_id`.

Ensure that you store the `user_token` along with your user's identifier in your database, as it is not possible to retrieve a previously created `user_token`.

See endpoint docs at <https://plaid.com/docs/api/users/#usercreate>.*/
    pub fn user_create(
        &self,
        client_user_id: &str,
    ) -> FluentRequest<'_, UserCreateRequest> {
        FluentRequest {
            client: self,
            params: UserCreateRequest {
                client_user_id: client_user_id.to_owned(),
                consumer_report_user_identity: None,
                end_customer: None,
            },
        }
    }
}
