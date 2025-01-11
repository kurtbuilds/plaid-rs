use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_initiation_recipient_list`].

On request success, this will return a [`PaymentInitiationRecipientListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListRequest {
    pub body: serde_json::Value,
}
impl FluentRequest<'_, PaymentInitiationRecipientListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationRecipientListRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationRecipientListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/recipient/list";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "body" : self.params.body }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**List payment recipients

The `/payment_initiation/recipient/list` endpoint list the payment recipients that you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientlist>.*/
    pub fn payment_initiation_recipient_list(
        &self,
        body: serde_json::Value,
    ) -> FluentRequest<'_, PaymentInitiationRecipientListRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationRecipientListRequest {
                body,
            },
        }
    }
}
