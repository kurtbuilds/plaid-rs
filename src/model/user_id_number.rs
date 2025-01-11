use serde::{Serialize, Deserialize};
use super::IdNumberType;
///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdNumber {
    ///A globally unique and human readable ID type, specific to the country and document category. For more context on this field, see [Hybrid Input Validation](https://plaid.com/docs/identity-verification/hybrid-input-validation).
    #[serde(rename = "type")]
    pub type_: IdNumberType,
    ///Value of identity document value typed in by user. Alpha-numeric, with all formatting characters stripped. For specific format requirements by ID type, see [Hybrid Input Validation](https://plaid.com/docs/identity-verification/hybrid-input-validation/).
    pub value: String,
}
impl std::fmt::Display for UserIdNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
