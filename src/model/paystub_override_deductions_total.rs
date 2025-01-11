use serde::{Serialize, Deserialize};
///An object representing the total deductions for the pay period
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideDeductionsTotal {
    ///The ISO-4217 currency code of the line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Raw amount of the deduction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///The year-to-date total amount of the deductions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PaystubOverrideDeductionsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
