use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_profile_get`].

On request success, this will return a [`PaymentProfileGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileGetRequest {
    pub payment_profile_token: String,
}
impl FluentRequest<'_, PaymentProfileGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PaymentProfileGetRequest> {
    type Output = httpclient::InMemoryResult<crate::model::PaymentProfileGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_profile/get";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    serde_json::json!(
                        { "payment_profile_token" : self.params.payment_profile_token }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get payment profile

Use `/payment_profile/get` endpoint to get the status of a given Payment Profile.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileget>.*/
    pub fn payment_profile_get(
        &self,
        payment_profile_token: &str,
    ) -> FluentRequest<'_, PaymentProfileGetRequest> {
        FluentRequest {
            client: self,
            params: PaymentProfileGetRequest {
                payment_profile_token: payment_profile_token.to_owned(),
            },
        }
    }
}
