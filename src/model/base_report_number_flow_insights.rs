use serde::{Serialize, Deserialize};
///The number of credits or debits out of the account. This field will only be included for depository accounts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportNumberFlowInsights {
    ///The number of credits or debits out of the account for this time period.
    pub count: i64,
    /**The end date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: chrono::NaiveDate,
    /**The start date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: chrono::NaiveDate,
}
impl std::fmt::Display for BaseReportNumberFlowInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
