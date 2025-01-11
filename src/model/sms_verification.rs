use serde::{Serialize, Deserialize};
use super::SmsVerificationStatus;
///Additional information for the individual SMS verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsVerification {
    ///The attempt field begins with 1 and increments with each subsequent SMS verification.
    pub attempt: i64,
    ///The number of delivery attempts made within the verification to send the SMS code to the user. Each delivery attempt represents the user taking action from the front end UI to request creation and delivery of a new SMS verification code, or to resend an existing SMS verification code. There is a limit of 3 delivery attempts per verification.
    pub delivery_attempt_count: i64,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initially_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    ///A phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    ///An ISO8601 formatted timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redacted_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The number of attempts made by the user within the verification to verify the SMS code by entering it into the front end UI. There is a limit of 3 solve attempts per verification.
    pub solve_attempt_count: i64,
    ///The outcome status for the individual SMS verification.
    pub status: SmsVerificationStatus,
}
impl std::fmt::Display for SmsVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
