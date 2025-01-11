use serde::{Serialize, Deserialize};
/**For a payment returned by this endpoint, there is only one possible value:

`PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentInitiationPaymentCreateStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    PaymentStatusInputNeeded,
}
