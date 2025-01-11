use serde::{Serialize, Deserialize};
use super::{ProfileIdentityAddress, ProfileIdentityName};
///ProfileIdentity defines the identity data permissioned by the end user during the authorization flow.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileIdentity {
    ///ProfileIdentityAddress defines the user's address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<ProfileIdentityAddress>,
    ///The user's date of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    ///The user's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///ProfileIdentityName defines the user's first name and last name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ProfileIdentityName>,
    ///The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    ///The user's social security number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    ///The last 4 digits of the user's social security number.
    #[serde(rename = "ssn_last_4")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn_last4: Option<String>,
}
impl std::fmt::Display for ProfileIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
