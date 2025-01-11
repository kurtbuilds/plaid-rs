use serde::{Serialize, Deserialize};
use super::LinkSessionSuccessMetadataInstitution;
///The details of a digital payroll income verification in Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionPayrollIncomeResult {
    ///An institution object. If the Item was created via Same-Day or Instant micro-deposit verification, will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionSuccessMetadataInstitution>,
    ///The number of paystubs retrieved from a payroll provider.
    pub num_paystubs_retrieved: i64,
    ///The number of W-2s retrieved from a payroll provider.
    #[serde(rename = "num_w2s_retrieved")]
    pub num_w2_s_retrieved: i64,
}
impl std::fmt::Display for LinkSessionPayrollIncomeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
