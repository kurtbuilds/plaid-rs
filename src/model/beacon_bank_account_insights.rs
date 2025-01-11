use serde::{Serialize, Deserialize};
use super::{AccountSubtype, AccountType, BeaconAccountRiskAttributes};
///Bank Account Insights encapsulate the risk insights for a single Bank Account linked to an Item that is assocaited with a Beacon User.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconBankAccountInsights {
    ///The Plaid `account_id`
    pub account_id: String,
    /**The attributes object contains data that can be used to assess account risk. Examples of data include:
`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
    pub attributes: BeaconAccountRiskAttributes,
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
    pub type_: AccountType,
}
impl std::fmt::Display for BeaconBankAccountInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
