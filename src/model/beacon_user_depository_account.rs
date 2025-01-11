use serde::{Serialize, Deserialize};
///Depository account information for the associated user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserDepositoryAccount {
    ///The last 2-4 numeric characters of this account’s account number.
    pub account_mask: String,
    ///An ISO8601 formatted timestamp.
    pub added_at: chrono::DateTime<chrono::Utc>,
    ///The routing number of the account.
    pub routing_number: String,
}
impl std::fmt::Display for BeaconUserDepositoryAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
