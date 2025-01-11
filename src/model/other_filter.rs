use serde::{Serialize, Deserialize};
use super::OtherAccountSubtypes;
///A filter to apply to `other`-type accounts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OtherFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: OtherAccountSubtypes,
}
impl std::fmt::Display for OtherFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
