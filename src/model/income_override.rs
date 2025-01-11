use serde::{Serialize, Deserialize};
use super::{PaystubOverride, W2Override};
///Specify payroll data on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeOverride {
    ///A list of paystubs associated with the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paystubs: Option<Vec<PaystubOverride>>,
    ///A list of w2s associated with the account.
    #[serde(rename = "w2s")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w2_s: Option<Vec<W2Override>>,
}
impl std::fmt::Display for IncomeOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
