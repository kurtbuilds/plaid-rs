use serde::{Serialize, Deserialize};
///The unit of the recurring interval.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferScheduleIntervalUnit {
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
}
