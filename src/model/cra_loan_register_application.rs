use serde::{Serialize, Deserialize};
///Contains loan application data to register.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraLoanRegisterApplication {
    ///The date the user applied for the loan. The date should be in ISO 8601 format (YYYY-MM-DD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_date: Option<chrono::NaiveDate>,
    /**A unique identifier for the loan application.
Personally identifiable information, such as an email address or phone number, should not be used in the `application_id`.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
impl std::fmt::Display for CraLoanRegisterApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
