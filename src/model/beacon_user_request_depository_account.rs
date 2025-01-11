use serde::{Serialize, Deserialize};
///Depository account information for the associated user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserRequestDepositoryAccount {
    ///Must be a valid US Bank Account Number
    pub account_number: String,
    ///The routing number of the account.
    pub routing_number: String,
}
impl std::fmt::Display for BeaconUserRequestDepositoryAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
