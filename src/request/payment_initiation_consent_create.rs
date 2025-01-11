use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    PaymentInitiationConsentConstraints, ExternalPaymentInitiationConsentOptions,
    PaymentInitiationConsentPayerDetails, PaymentInitiationConsentScope,
    PaymentInitiationConsentType,
};
/**You should use this struct via [`PlaidClient::payment_initiation_consent_create`].

On request success, this will return a [`PaymentInitiationConsentCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequest {
    pub constraints: PaymentInitiationConsentConstraints,
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
    pub payer_details: Option<PaymentInitiationConsentPayerDetails>,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Option<Vec<PaymentInitiationConsentScope>>,
    pub type_: Option<PaymentInitiationConsentType>,
}
impl FluentRequest<'_, PaymentInitiationConsentCreateRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: ExternalPaymentInitiationConsentOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the payer_details field.
    pub fn payer_details(
        mut self,
        payer_details: PaymentInitiationConsentPayerDetails,
    ) -> Self {
        self.params.payer_details = Some(payer_details);
        self
    }
    ///Set the value of the scopes field.
    pub fn scopes(mut self, scopes: Vec<PaymentInitiationConsentScope>) -> Self {
        self.params.scopes = Some(scopes);
        self
    }
    ///Set the value of the type_ field.
    pub fn type_(mut self, type_: PaymentInitiationConsentType) -> Self {
        self.params.type_ = Some(type_);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationConsentCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationConsentCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/consent/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "constraints" : self.params.constraints }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.payer_details {
                r = r.json(serde_json::json!({ "payer_details" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "recipient_id" : self.params.recipient_id }));
            r = r.json(serde_json::json!({ "reference" : self.params.reference }));
            if let Some(ref unwrapped) = self.params.scopes {
                r = r.json(serde_json::json!({ "scopes" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.type_ {
                r = r.json(serde_json::json!({ "type" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create payment consent

The `/payment_initiation/consent/create` endpoint is used to create a payment consent, which can be used to initiate payments on behalf of the user. Payment consents are created with `UNAUTHORISED` status by default and must be authorised by the user before payments can be initiated.

Consents can be limited in time and scope, and have constraints that describe limitations for payments.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentcreate>.*/
    pub fn payment_initiation_consent_create(
        &self,
        constraints: PaymentInitiationConsentConstraints,
        recipient_id: &str,
        reference: &str,
    ) -> FluentRequest<'_, PaymentInitiationConsentCreateRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationConsentCreateRequest {
                constraints,
                options: None,
                payer_details: None,
                recipient_id: recipient_id.to_owned(),
                reference: reference.to_owned(),
                scopes: None,
                type_: None,
            },
        }
    }
}
