use serde::{Serialize, Deserialize};
use super::{PaystubOverrideEarningsBreakdown, PaystubOverrideEarningsTotal};
///An object representing both a breakdown of earnings on a paystub and the total earnings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEarnings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<Vec<PaystubOverrideEarningsBreakdown>>,
    ///An object representing both the current pay period and year to date amount for an earning category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<PaystubOverrideEarningsTotal>,
}
impl std::fmt::Display for PaystubOverrideEarnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
