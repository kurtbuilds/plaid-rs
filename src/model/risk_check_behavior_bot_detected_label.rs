use serde::{Serialize, Deserialize};
/**Field describing the outcome of a bot detection behavior risk check.

`yes` indicates that automated activity was detected.

`no` indicates that automated activity was not detected.

`no_data` indicates there was not enough information available to give an accurate signal.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckBehaviorBotDetectedLabel {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
