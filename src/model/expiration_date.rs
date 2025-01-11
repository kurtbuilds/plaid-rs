use serde::{Serialize, Deserialize};
/**A description of whether the associated document was expired when the verification was performed.

Note: In the case where an expiration date is not present on the document or failed to be extracted, this value will be `no_data`.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExpirationDate {
    #[serde(rename = "not_expired")]
    NotExpired,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "no_data")]
    NoData,
}
