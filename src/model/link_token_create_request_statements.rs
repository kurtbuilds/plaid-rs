use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Statements product. This field is required for the statements product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestStatements {
    ///The end date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) "YYYY-MM-DD" format, e.g. "2020-10-30". You can request up to two years of data.
    pub end_date: chrono::NaiveDate,
    ///The start date for statements, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) "YYYY-MM-DD" format, e.g. "2020-10-30".
    pub start_date: chrono::NaiveDate,
}
impl std::fmt::Display for LinkTokenCreateRequestStatements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
