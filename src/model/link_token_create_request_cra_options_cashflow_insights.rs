use serde::{Serialize, Deserialize};
use super::CashflowAttributesVersion;
///Specifies options for initializing Link for use with the Cashflow Insights product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestCraOptionsCashflowInsights {
    ///The versions of cashflow attributes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes_version: Option<CashflowAttributesVersion>,
    ///The version of the Plaid Check score to return
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_check_score_version: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestCraOptionsCashflowInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
