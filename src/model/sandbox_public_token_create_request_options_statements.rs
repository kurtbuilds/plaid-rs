use serde::{Serialize, Deserialize};
///An optional set of parameters corresponding to statements options.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxPublicTokenCreateRequestOptionsStatements {
    ///The most recent date for which to fetch statements history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: chrono::NaiveDate,
    ///The earliest date for which to fetch statements history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: chrono::NaiveDate,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsStatements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
