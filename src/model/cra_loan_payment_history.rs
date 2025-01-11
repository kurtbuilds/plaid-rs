use serde::{Serialize, Deserialize};
///Contains the payment information for a loan payment period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraLoanPaymentHistory {
    ///The amount past due or the charge-off amount of the loan at the end of the payment period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_past_due: Option<f64>,
    ///The balance remaining on the loan at the end of the payment period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_remaining: Option<f64>,
    /**The number of days the loan was delinquent at the end of the pay period.
If specified, should be greater of equal to 0.*/
    pub days_past_due: i64,
    ///The payment due date or end date of the payment period. The date should be in ISO 8601 format (YYYY-MM-DD).
    pub due_date: chrono::NaiveDate,
    /**The index to identify the loan's payment period, starting from 1.
For example:
  1 means the period between the loan's opening date and the 1st payment due date.
  2 means the period between the loan's 1st payment due date and 2nd payment due date.*/
    pub period: i64,
}
impl std::fmt::Display for CraLoanPaymentHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
