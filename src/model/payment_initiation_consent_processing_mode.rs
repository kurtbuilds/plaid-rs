use serde::{Serialize, Deserialize};
/**Decides the mode under which the payment processing should be performed, using `IMMEDIATE` as default.

`IMMEDIATE`: Will immediately execute the payment, waiting for a response from the ASPSP before returning the result of the payment initiation. This is ideal for user-present flows.

 `ASYNC`: Will accept a payment execution request and schedule it for processing, immediately returning the new `payment_id`. Listen for webhooks or use the [`/payment_initiation/payment/get`](https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentget) endpoint to obtain updates on the payment status. This is ideal for non user-present flows.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentInitiationConsentProcessingMode {
    #[serde(rename = "ASYNC")]
    Async,
    #[serde(rename = "IMMEDIATE")]
    Immediate,
}
