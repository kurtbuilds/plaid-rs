use serde::{Serialize, Deserialize};
use super::BeaconMatchSummaryCode;
///Analysis of whether this account matched between the originally reported Beacon User and the Beacon User that the report syndicated to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconSyndicatedReportDepositoryAccountMatchAnalysis {
    ///The last 2-4 numeric characters of this account’s account number.
    pub account_mask: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub match_status: BeaconMatchSummaryCode,
    ///The routing number of the account.
    pub routing_number: String,
}
impl std::fmt::Display for BeaconSyndicatedReportDepositoryAccountMatchAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
