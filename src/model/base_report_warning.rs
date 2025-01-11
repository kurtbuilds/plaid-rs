use serde::{Serialize, Deserialize};
use super::{BaseReportWarningCode, Cause};
///It is possible for a Base Report to be returned with missing account owner information. In such cases, the Base Report will contain warning data in the response, indicating why obtaining the owner information failed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseReportWarning {
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<Cause>,
    ///The warning code identifies a specific kind of warning. `OWNERS_UNAVAILABLE` indicates that account-owner information is not available. `TRANSACTIONS_UNAVAILABLE` indicates that transactions information associated with Credit and Depository accounts are unavailable. `USER_FRAUD_ALERT` indicates that the User has placed a fraud alert on their Plaid Check consumer report due to suspected fraud.
    pub warning_code: BaseReportWarningCode,
    ///The warning type, which will always be `BASE_REPORT_WARNING`
    pub warning_type: String,
}
impl std::fmt::Display for BaseReportWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
