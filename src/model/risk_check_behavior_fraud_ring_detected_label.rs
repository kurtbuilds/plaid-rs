use serde::{Serialize, Deserialize};
/**Field describing the outcome of a fraud ring behavior risk check.

`yes` indicates that fraud ring activity was detected.

`no` indicates that fraud ring activity was not detected.

`no_data` indicates there was not enough information available to give an accurate signal.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckBehaviorFraudRingDetectedLabel {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
