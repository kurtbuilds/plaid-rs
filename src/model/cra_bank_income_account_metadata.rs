use serde::{Serialize, Deserialize};
///An object containing metadata about the extracted account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeAccountMetadata {
    ///The date of the most recent extracted transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The date of the earliest extracted transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for CraBankIncomeAccountMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
