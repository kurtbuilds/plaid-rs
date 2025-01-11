use serde::{Serialize, Deserialize};
use super::{
    AccountVerificationInsightsAccountNumberFormat,
    AccountVerificationInsightsNetworkStatus, AccountVerificationInsightsPreviousReturns,
};
///Insights from performing database verification for the account. Only returned for Auth Items created via Database Insights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountVerificationInsights {
    /**Indicator of account number format validity for institution.

`valid`: indicates that the account number has a correct format for the institution.

`invalid`: indicates that the account number has an incorrect format for the institution.

`unknown`: indicates that there was not enough information to determine whether the format is correct for the institution.*/
    pub account_number_format: AccountVerificationInsightsAccountNumberFormat,
    ///Status information about the account and routing number in the Plaid network.
    pub network_status: AccountVerificationInsightsNetworkStatus,
    ///Information about known ACH returns for the account and routing number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_returns: Option<AccountVerificationInsightsPreviousReturns>,
}
impl std::fmt::Display for AccountVerificationInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
