use serde::{Serialize, Deserialize};
use super::{CraLoanApplicationDecision, CraLoanType};
///Contains loan application data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraLoanApplication {
    ///The date the user applied for the loan. The date should be in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_date: Option<chrono::NaiveDate>,
    /**A unique identifier for the loan application.
Personally identifiable information, such as an email address or phone number, should not be used in the `application_id`.*/
    pub application_id: String,
    ///The decision of the loan application.
    pub decision: CraLoanApplicationDecision,
    ///The date when the loan application's decision was made. The date should be in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_date: Option<chrono::NaiveDate>,
    ///The type of loan the user applied for.
    #[serde(rename = "type")]
    pub type_: CraLoanType,
    ///The user token for the user associated with the loan.
    pub user_token: String,
}
impl std::fmt::Display for CraLoanApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
