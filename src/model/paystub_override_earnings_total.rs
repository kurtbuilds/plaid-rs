use serde::{Serialize, Deserialize};
///An object representing both the current pay period and year to date amount for an earning category.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEarningsTotal {
    ///The ISO-4217 currency code of the line item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Total number of hours worked for this pay period
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    ///The year-to-date amount for the total earnings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PaystubOverrideEarningsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
