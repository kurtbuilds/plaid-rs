use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_initiation_payment_get`].

On request success, this will return a [`PaymentInitiationPaymentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetRequest {
    pub payment_id: String,
}
impl FluentRequest<'_, PaymentInitiationPaymentGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationPaymentGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/payment/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "payment_id" : self.params.payment_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get payment details

The `/payment_initiation/payment/get` endpoint can be used to check the status of a payment, as well as to receive basic information such as recipient and payment amount. In the case of standing orders, the `/payment_initiation/payment/get` endpoint will provide information about the status of the overall standing order itself; the API cannot be used to retrieve payment status for individual payments within a standing order.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentget>.*/
    pub fn payment_initiation_payment_get(
        &self,
        payment_id: &str,
    ) -> FluentRequest<'_, PaymentInitiationPaymentGetRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationPaymentGetRequest {
                payment_id: payment_id.to_owned(),
            },
        }
    }
}
