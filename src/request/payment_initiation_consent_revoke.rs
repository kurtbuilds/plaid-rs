use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
/**You should use this struct via [`PlaidClient::payment_initiation_consent_revoke`].

On request success, this will return a [`PaymentInitiationConsentRevokeResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentRevokeRequest {
    pub consent_id: String,
}
impl FluentRequest<'_, PaymentInitiationConsentRevokeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentRevokeRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationConsentRevokeResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/revoke";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "consent_id" : self.params.consent_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Revoke payment consent

The `/payment_initiation/consent/revoke` endpoint can be used to revoke the payment consent. Once the consent is revoked, it is not possible to initiate payments using it.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentrevoke>.*/
    pub fn payment_initiation_consent_revoke(
        &self,
        consent_id: &str,
    ) -> FluentRequest<'_, PaymentInitiationConsentRevokeRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationConsentRevokeRequest {
                consent_id: consent_id.to_owned(),
            },
        }
    }
}
