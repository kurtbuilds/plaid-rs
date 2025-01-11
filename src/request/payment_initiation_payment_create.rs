use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::model::{
    PaymentAmount, ExternalPaymentOptions, ExternalPaymentScheduleRequest,
};
/**You should use this struct via [`PlaidClient::payment_initiation_payment_create`].

On request success, this will return a [`PaymentInitiationPaymentCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    pub amount: PaymentAmount,
    pub options: Option<ExternalPaymentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub schedule: Option<ExternalPaymentScheduleRequest>,
}
impl FluentRequest<'_, PaymentInitiationPaymentCreateRequest> {
    ///Set the value of the options field.
    pub fn options(mut self, options: ExternalPaymentOptions) -> Self {
        self.params.options = Some(options);
        self
    }
    ///Set the value of the schedule field.
    pub fn schedule(mut self, schedule: ExternalPaymentScheduleRequest) -> Self {
        self.params.schedule = Some(schedule);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationPaymentCreateRequest> {
    type Output = httpclient::InMemoryResult<
        crate::model::PaymentInitiationPaymentCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/payment/create";
            let mut r = self.client.client.post(url);
            r = r.json(serde_json::json!({ "amount" : self.params.amount }));
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(serde_json::json!({ "options" : unwrapped }));
            }
            r = r.json(serde_json::json!({ "recipient_id" : self.params.recipient_id }));
            r = r.json(serde_json::json!({ "reference" : self.params.reference }));
            if let Some(ref unwrapped) = self.params.schedule {
                r = r.json(serde_json::json!({ "schedule" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
impl crate::PlaidClient {
    /**Create a payment

After creating a payment recipient, you can use the `/payment_initiation/payment/create` endpoint to create a payment to that recipient.  Payments can be one-time or standing order (recurring) and can be denominated in either EUR, GBP or other chosen [currency](https://plaid.com/docs/api/products/payment-initiation/#payment_initiation-payment-create-request-amount-currency).  If making domestic GBP-denominated payments, your recipient must have been created with BACS numbers. In general, EUR-denominated payments will be sent via SEPA Credit Transfer, GBP-denominated payments will be sent via the Faster Payments network and for non-Eurozone markets typically via the local payment scheme, but the payment network used will be determined by the institution. Payments sent via Faster Payments will typically arrive immediately, while payments sent via SEPA Credit Transfer or other local payment schemes will typically arrive in one business day.

Standing orders (recurring payments) must be denominated in GBP and can only be sent to recipients in the UK. Once created, standing order payments cannot be modified or canceled via the API. An end user can cancel or modify a standing order directly on their banking application or website, or by contacting the bank. Standing orders will follow the payment rules of the underlying rails (Faster Payments in UK). Payments can be sent Monday to Friday, excluding bank holidays. If the pre-arranged date falls on a weekend or bank holiday, the payment is made on the next working day. It is not possible to guarantee the exact time the payment will reach the recipient’s account, although at least 90% of standing order payments are sent by 6am.

In Limited Production, payments must be below 5 GBP or other chosen [currency](https://plaid.com/docs/api/products/payment-initiation/#payment_initiation-payment-create-request-amount-currency), and standing orders, variable recurring payments, and Virtual Accounts are not supported.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentcreate>.*/
    pub fn payment_initiation_payment_create(
        &self,
        amount: PaymentAmount,
        recipient_id: &str,
        reference: &str,
    ) -> FluentRequest<'_, PaymentInitiationPaymentCreateRequest> {
        FluentRequest {
            client: self,
            params: PaymentInitiationPaymentCreateRequest {
                amount,
                options: None,
                recipient_id: recipient_id.to_owned(),
                reference: reference.to_owned(),
                schedule: None,
            },
        }
    }
}
