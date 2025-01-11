use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_profile_remove`].

On request success, this will return a [`PaymentProfileRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProfileRemoveRequest {
    pub payment_profile_token: String,
}
impl FluentRequest<'_, PaymentProfileRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PaymentProfileRemoveRequest> {
    type Output = httpclient::InMemoryResult<crate::model::PaymentProfileRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_profile/remove";
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
    /**Remove payment profile

Use the `/payment_profile/remove` endpoint to remove a given Payment Profile. Once it’s removed, it can no longer be used to create transfers.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileremove>.*/
    pub fn payment_profile_remove(
        &self,
        payment_profile_token: &str,
    ) -> FluentRequest<'_, PaymentProfileRemoveRequest> {
        FluentRequest {
            client: self,
            params: PaymentProfileRemoveRequest {
                payment_profile_token: payment_profile_token.to_owned(),
            },
        }
    }
}
