use serde::{Serialize, Deserialize};
///Information about known ACH returns for the account and routing number.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountVerificationInsightsPreviousReturns {
    ///Indicates whether Plaid's data sources include a known administrative ACH return for account and routing number.
    pub has_previous_administrative_return: bool,
}
impl std::fmt::Display for AccountVerificationInsightsPreviousReturns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
