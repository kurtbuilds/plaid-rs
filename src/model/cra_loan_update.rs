use serde::{Serialize, Deserialize};
use super::{CraLoanPaymentHistory, CraLoanStatusHistoryUpdate};
///Contains loan data to update.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraLoanUpdate {
    /**A unique identifier for the loan.
Personally identifiable information, such as an email address or phone number, should not be used in the `loan_id`.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    ///The updates to the payment history for the loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_history: Option<Vec<CraLoanPaymentHistory>>,
    ///A list of status update history of the loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<CraLoanStatusHistoryUpdate>>,
}
impl std::fmt::Display for CraLoanUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
