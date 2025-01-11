use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_profile_create`].

On request success, this will return a [`PaymentProfileCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileCreateRequest {
    pub body: serde_json::Value,
}
impl FluentRequest<'_, PaymentProfileCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PaymentProfileCreateRequest> {
    type Output = httpclient::InMemoryResult<crate::model::PaymentProfileCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_profile/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create payment profile

Use `/payment_profile/create` endpoint to create a new payment profile.
To initiate the account linking experience, call `/link/token/create` and provide the `payment_profile_token` in the `transfer.payment_profile_token` field.
You can then use the `payment_profile_token` when creating transfers using `/transfer/authorization/create` and `/transfer/create`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profilecreate>.*/
    pub fn payment_profile_create(
        &self,
        body: serde_json::Value,
    ) -> FluentRequest<'_, PaymentProfileCreateRequest> {
        FluentRequest {
            client: self,
            params: PaymentProfileCreateRequest {
                body,
            },
        }
    }
}
