use serde::{Serialize, Deserialize};
use super::AddressData;
///To create a Plaid Check Consumer Report for a user, this field must be present on the user token. If this field is not provided during user token creation, you can add it to the user later by calling `/user/update`. Once the field has been added to the user, you will be able to call `/link/token/create` with a non-empty `consumer_report_permissible_purpose` (which will automatically create a Plaid Check Consumer Report), or call `/cra/check_report/create` for that user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsumerReportUserIdentity {
    /**To be provided in the format "yyyy-mm-dd".
This field is required for any clients who became Plaid Check customers on or after Oct 1, 2024.
This field will be required for all Plaid Check customers as of Feb 1, 2025.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///The user's emails
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    ///The user's first name
    pub first_name: String,
    ///The user's last name
    pub last_name: String,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14157452130". Phone numbers provided in other formats will be parsed on a best-effort basis. Phone number input is validated against valid number ranges; number strings that do not match a real-world phone numbering scheme may cause the request to fail, even in the Sandbox test environment.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phone_numbers: Vec<String>,
    ///Data about the components comprising an address.
    pub primary_address: AddressData,
    ///The last 4 digits of the user's social security number.
    #[serde(rename = "ssn_last_4")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn_last4: Option<String>,
}
impl std::fmt::Display for ConsumerReportUserIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
