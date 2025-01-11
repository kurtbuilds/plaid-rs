use serde::{Serialize, Deserialize};
use super::{
    CraLoanOpenedStatus, CraLoanPaymentSchedule, CraLoanRegisterApplication, CraLoanType,
};
///Contains loan data to register.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoanRegister {
    ///Contains loan application data to register.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<CraLoanRegisterApplication>,
    ///The total amount of the approved loan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan_amount: Option<f64>,
    /**A unique identifier for the loan.
Personally identifiable information, such as an email address or phone number, should not be used in the `loan_id`.*/
    pub loan_id: String,
    ///The date the loan account was opened. The date should be in ISO 8601 format (YYYY-MM-DD).
    pub opened_date: chrono::NaiveDate,
    ///Contains the status and date information of the loan when registering.
    pub opened_with_status: CraLoanOpenedStatus,
    /**The frequency of a loan's payment schedule.
`BIWEEKLY` represents one payment every two weeks.*/
    pub payment_schedule: CraLoanPaymentSchedule,
    ///The type of loan the user applied for.
    #[serde(rename = "type")]
    pub type_: CraLoanType,
    ///The user token for the user associated with the loan.
    pub user_token: String,
}
impl std::fmt::Display for CraLoanRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
