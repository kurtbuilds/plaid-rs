use serde::{Serialize, Deserialize};
use super::DepositoryAccountSubtypes;
///A filter to apply to `depository`-type accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositoryFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: DepositoryAccountSubtypes,
}
impl std::fmt::Display for DepositoryFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
