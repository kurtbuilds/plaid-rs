use serde::{Serialize, Deserialize};
///The billing contact for the end customer. Defaults to partner's billing contact if omitted.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerEndCustomerBillingContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerBillingContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
