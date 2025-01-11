use serde::{Serialize, Deserialize};
use super::EarningsBreakdownCanonicalDescription;
///An object representing the earnings line items for the pay period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEarningsBreakdown {
    ///Commonly used term to describe the earning line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_description: Option<EarningsBreakdownCanonicalDescription>,
    ///The ISO-4217 currency code of the line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Raw amount of the earning line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    ///Description of the earning line item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Number of hours applicable for this earning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    ///Hourly rate applicable for this earning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    ///The year-to-date amount of the deduction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PaystubOverrideEarningsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
