use serde::{Serialize, Deserialize};
use super::BeaconBankAccountInsights;
///A collection of Bank Accounts linked to an Item that is associated with this Beacon User.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconBankAccounts {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<BeaconBankAccountInsights>,
    ///The Plaid Item ID the Bank Accounts belong to.
    pub item_id: String,
}
impl std::fmt::Display for BeaconBankAccounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
