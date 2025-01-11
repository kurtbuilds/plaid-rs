use serde::{Serialize, Deserialize};
///Payment consent periodic interval.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentConsentPeriodicInterval {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "YEAR")]
    Year,
}
