use serde::{Serialize, Deserialize};
///Configuration parameters for the Investments Move product
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenInvestmentsAuth {
    ///If `true`, show institutions that use the manual entry fallback flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_entry_enabled: Option<bool>,
    ///If `true`, show institutions that use the masked number match fallback flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub masked_number_match_enabled: Option<bool>,
    ///If `true`, show institutions that use the stated account number fallback flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stated_account_number_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenInvestmentsAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
