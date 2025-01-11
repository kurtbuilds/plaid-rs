use serde::{Serialize, Deserialize};
///The frequency interval of the payment.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentScheduleInterval {
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
}
