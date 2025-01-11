use serde::{Serialize, Deserialize};
use super::{AccountSubtype, AccountType};
///A financial institution account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentedAccount {
    ///Plaid’s unique identifier for the account. Like all Plaid identifiers, the `account_id` is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///The last 2-4 alphanumeric characters of an account's official account number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///The name of the account, either assigned by the user or by the financial institution itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The official name of the account as given by the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<AccountSubtype>,
    /**`investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AccountType>,
}
impl std::fmt::Display for ConsentedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
