use serde::{Serialize, Deserialize};
use super::{
    BeaconUserIdNumber, BeaconUserName, BeaconUserRequestAddress,
    BeaconUserRequestDepositoryAccount,
};
/**A Beacon User's data which is used to check against duplicate records and the Beacon Fraud Network.

In order to create a Beacon User, in addition to the `name`, _either_ the `date_of_birth` _or_ the `depository_accounts` field must be provided.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconUserRequestData {
    ///Home address for the associated user. For more context on this field, see [Input Validation by Country](https://plaid.com/docs/identity-verification/hybrid-input-validation/#input-validation-by-country).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<BeaconUserRequestAddress>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    /**Provide a list of bank accounts that are associated with this Beacon User. These accounts will be scanned across the Beacon Network and used to find duplicate records.

Note: These accounts will not have Bank Account Insights. To receive Bank Account Insights please supply `access_tokens`.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depository_accounts: Option<Vec<BeaconUserRequestDepositoryAccount>>,
    ///A valid email address. Must not have leading or trailing spaces and address must be RFC compliant. For more information, see [RFC 3696](https://datatracker.ietf.org/doc/html/rfc3696).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    ///The ID number associated with a Beacon User.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<BeaconUserIdNumber>,
    ///An IPv4 or IPV6 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The full name for a given Beacon User.
    pub name: BeaconUserName,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl std::fmt::Display for BeaconUserRequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
