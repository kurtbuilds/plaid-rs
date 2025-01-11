use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_initiation_consent_get`].

On request success, this will return a [`PaymentInitiationConsentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentGetRequest {
    pub consent_id: String,
}
impl FluentRequest<'_, PaymentInitiationConsentGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentGetRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationConsentGetResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/get";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "consent_id" : self.params.consent_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Get payment consent

The `/payment_initiation/consent/get` endpoint can be used to check the status of a payment consent, as well as to receive basic information such as recipient and constraints.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentget>.*/
    pub fn payment_initiation_consent_get(
        &self,
        consent_id: &str,
    ) -> FluentRequest<'_, PaymentInitiationConsentGetRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationConsentGetRequest {
                consent_id: consent_id.to_owned(),
            },
        }
    }
}
