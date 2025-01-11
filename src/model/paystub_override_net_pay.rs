use serde::{Serialize, Deserialize};
///An object representing information about the net pay amount on the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideNetPay {
    ///The ISO-4217 currency code of the net pay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Description of the net pay
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The year-to-date amount of the net pay
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PaystubOverrideNetPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
