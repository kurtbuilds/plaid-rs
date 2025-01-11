use serde::{Serialize, Deserialize};
use super::CraLoanClosedStatus;
///Contains loan data for the loan being unregistered.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoanUnregister {
    ///Contains the status and date information of the loan when unregistering.
    pub closed_with_status: CraLoanClosedStatus,
    /**A unique identifier for the loan.
Personally identifiable information, such as an email address or phone number, should not be used in the `loan_id`.*/
    pub loan_id: String,
}
impl std::fmt::Display for CraLoanUnregister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
