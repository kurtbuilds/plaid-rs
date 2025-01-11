use serde::{Serialize, Deserialize};
use super::{
    CreditSessionDocumentIncomeResult, LinkSessionBankIncomeResult,
    LinkSessionCraItemAddResult, LinkSessionItemAddResult, LinkSessionPayrollIncomeResult,
};
///The set of results for a Link session.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionResults {
    ///The set of bank income verifications for the Link session.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_income_results: Vec<LinkSessionBankIncomeResult>,
    ///The set of Plaid Check Item adds for the Link session.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cra_item_add_results: Vec<LinkSessionCraItemAddResult>,
    ///The details of a document income verification in Link
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_income_results: Option<CreditSessionDocumentIncomeResult>,
    ///The set of Item adds for the Link session.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_add_results: Vec<LinkSessionItemAddResult>,
    ///The set of payroll income verifications for the Link session.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payroll_income_results: Vec<LinkSessionPayrollIncomeResult>,
}
impl std::fmt::Display for LinkSessionResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
