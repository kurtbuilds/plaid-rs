use serde::{Serialize, Deserialize};
use super::CraLoanStatus;
///Contains the status and date of an update to the loan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoanStatusHistoryUpdate {
    ///The effective date for the status of the loan. The date should be in ISO 8601 format (YYYY-MM-DD).
    pub date: chrono::NaiveDate,
    ///The status of the loan.
    pub status: CraLoanStatus,
}
impl std::fmt::Display for CraLoanStatusHistoryUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
