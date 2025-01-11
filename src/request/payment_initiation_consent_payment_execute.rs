use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{PaymentAmount, PaymentInitiationConsentProcessingMode};
/**You should use this struct via [`PlaidClient::payment_initiation_consent_payment_execute`].

On request success, this will return a [`PaymentInitiationConsentPaymentExecuteResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteRequest {
    pub amount: PaymentAmount,
    pub consent_id: String,
    pub idempotency_key: String,
    pub processing_mode: Option<PaymentInitiationConsentProcessingMode>,
    pub reference: Option<String>,
    pub scope: Option<serde_json::Value>,
}
impl FluentRequest<'_, PaymentInitiationConsentPaymentExecuteRequest> {
    ///Set the value of the processing_mode field.
    pub fn processing_mode(
        mut self,
        processing_mode: PaymentInitiationConsentProcessingMode,
    ) -> Self {
        self.params.processing_mode = Some(processing_mode);
        self
    }
    ///Set the value of the reference field.
    pub fn reference(mut self, reference: &str) -> Self {
        self.params.reference = Some(reference.to_owned());
        self
    }
    ///Set the value of the scope field.
    pub fn scope(mut self, scope: serde_json::Value) -> Self {
        self.params.scope = Some(scope);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentPaymentExecuteRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationConsentPaymentExecuteResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/payment/execute";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            r = r.json(serde_json::json!({ "consent_id" : self.params.consent_id }));
            r = r
                .json(
                    serde_json::json!(
                        { "idempotency_key" : self.params.idempotency_key }
                    ),
                );
            if let Some(ref unwrapped) = self.params.processing_mode {
                r = r.json(serde_json::json!({ "processing_mode" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.reference {
                r = r.json(serde_json::json!({ "reference" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.scope {
                r = r.json(serde_json::json!({ "scope" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Execute a single payment using consent

The `/payment_initiation/consent/payment/execute` endpoint can be used to execute payments using payment consent.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentpaymentexecute>.*/
    pub fn payment_initiation_consent_payment_execute(
        &self,
        amount: PaymentAmount,
        consent_id: &str,
        idempotency_key: &str,
    ) -> FluentRequest<'_, PaymentInitiationConsentPaymentExecuteRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationConsentPaymentExecuteRequest {
                amount,
                consent_id: consent_id.to_owned(),
                idempotency_key: idempotency_key.to_owned(),
                processing_mode: None,
                reference: None,
                scope: None,
            },
        }
    }
}
