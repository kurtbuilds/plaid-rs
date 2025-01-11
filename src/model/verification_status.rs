use serde::{Serialize, Deserialize};
/**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerificationStatus {
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "NEEDS_INFO")]
    NeedsInfo,
    #[serde(rename = "UNABLE_TO_VERIFY")]
    UnableToVerify,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
