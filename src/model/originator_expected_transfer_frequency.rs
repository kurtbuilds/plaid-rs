use serde::{Serialize, Deserialize};
///The originator's expected transfer frequency.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OriginatorExpectedTransferFrequency {
    #[serde(rename = "once_per_month")]
    OncePerMonth,
    #[serde(rename = "twice_per_month")]
    TwicePerMonth,
    #[serde(rename = "once_per_week")]
    OncePerWeek,
    #[serde(rename = "daily")]
    Daily,
}
