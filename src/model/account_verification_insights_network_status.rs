use serde::{Serialize, Deserialize};
///Status information about the account and routing number in the Plaid network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountVerificationInsightsNetworkStatus {
    ///Indicates whether we found at least one matching account for the ACH account and routing number.
    pub has_numbers_match: bool,
    ///Indicates if at least one matching account for the ACH account and routing number is already verified.
    pub is_numbers_match_verified: bool,
}
impl std::fmt::Display for AccountVerificationInsightsNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
