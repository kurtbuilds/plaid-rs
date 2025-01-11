use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::create_payment_token`].

On request success, this will return a [`PaymentInitiationPaymentTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentTokenRequest {
    pub payment_id: String,
}
impl FluentRequest<'_, CreatePaymentTokenRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreatePaymentTokenRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationPaymentTokenCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/payment/token/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "payment_id" : self.params.payment_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create payment token

The `/payment_initiation/payment/token/create` endpoint has been deprecated. New Plaid customers will be unable to use this endpoint, and existing customers are encouraged to migrate to the newer, `link_token`-based flow. The recommended flow is to provide the `payment_id` to `/link/token/create`, which returns a `link_token` used to initialize Link.

The `/payment_initiation/payment/token/create` is used to create a `payment_token`, which can then be used in Link initialization to enter a payment initiation flow. You can only use a `payment_token` once. If this attempt fails, the end user aborts the flow, or the token expires, you will need to create a new payment token. Creating a new payment token does not require end user input.

See endpoint docs at <https://plaid.com/docs/link/maintain-legacy-integration/#creating-a-payment-token>.*/
    pub fn create_payment_token(
        &self,
        payment_id: &str,
    ) -> FluentRequest<'_, CreatePaymentTokenRequest> {
        FluentRequest {
            client: self,
            params: CreatePaymentTokenRequest {
                payment_id: payment_id.to_owned(),
            },
        }
    }
}
