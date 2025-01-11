use serde::{Serialize, Deserialize};
///An object representing the deduction line items for the pay period
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideDeductionsBreakdown {
    ///The ISO-4217 currency code of the line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Raw amount of the deduction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///Description of the deduction line item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The year-to-date amount of the deduction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PaystubOverrideDeductionsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
