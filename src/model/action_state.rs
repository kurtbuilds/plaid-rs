use serde::{Serialize, Deserialize};
///Enum representing the state of the action/activity.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionState {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ATTEMPT")]
    Attempt,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "SKIPPED")]
    Skipped,
}
