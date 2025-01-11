use serde::{Serialize, Deserialize};
use super::{UserAccountIdentityAddress, UserAccountIdentityName};
///The identity data permissioned by the end user during the authorization flow.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAccountIdentity {
    ///The user's address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<UserAccountIdentityAddress>,
    ///The user's date of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /**The user's email address.

Note: email is currently not returned for users, and is an upcoming addition that will be live in early 2025.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///The user's first name and last name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UserAccountIdentityName>,
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
impl std::fmt::Display for UserAccountIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
