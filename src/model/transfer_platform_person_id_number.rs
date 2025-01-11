use serde::{Serialize, Deserialize};
use super::IdNumberType;
///ID number of the person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPlatformPersonIdNumber {
    ///A globally unique and human readable ID type, specific to the country and document category. For more context on this field, see [Hybrid Input Validation](https://plaid.com/docs/identity-verification/hybrid-input-validation).
    #[serde(rename = "type")]
    pub type_: IdNumberType,
    ///Value of the person's ID Number. Alpha-numeric, with all formatting characters stripped.
    pub value: String,
}
impl std::fmt::Display for TransferPlatformPersonIdNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
