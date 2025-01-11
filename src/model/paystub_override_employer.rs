use serde::{Serialize, Deserialize};
use super::PaystubOverrideEmployerAddress;
///The employer on the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEmployer {
    ///The address of the employer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaystubOverrideEmployerAddress>,
    ///The name of the employer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
