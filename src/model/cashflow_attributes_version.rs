use serde::{Serialize, Deserialize};
///The versions of cashflow attributes
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CashflowAttributesVersion {
    #[serde(rename = "v1.0")]
    V10,
}
