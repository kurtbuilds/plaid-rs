use serde::{Serialize, Deserialize};
use super::{PaystubOverrideEmployeeAddress, PaystubOverrideTaxpayerId};
///The employee on the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEmployee {
    ///The address of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaystubOverrideEmployeeAddress>,
    ///Marital status of the employee - either `single` or `married`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    ///The name of the employee.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Taxpayer ID of the individual receiving the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxpayer_id: Option<PaystubOverrideTaxpayerId>,
}
impl std::fmt::Display for PaystubOverrideEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
