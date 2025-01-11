use serde::{Serialize, Deserialize};
use super::{PaystubOverrideDeductionsBreakdown, PaystubOverrideDeductionsTotal};
///An object with the deduction information found on a paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideDeductions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<PaystubOverrideDeductionsBreakdown>>,
    ///An object representing the total deductions for the pay period
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<PaystubOverrideDeductionsTotal>,
}
impl std::fmt::Display for PaystubOverrideDeductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
