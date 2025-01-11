use serde::{Serialize, Deserialize};
///Taxpayer ID of the individual receiving the paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideTaxpayerId {
    ///ID mask; i.e. last 4 digits of the taxpayer ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_mask: Option<String>,
    ///Type of ID, e.g. 'SSN'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}
impl std::fmt::Display for PaystubOverrideTaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
