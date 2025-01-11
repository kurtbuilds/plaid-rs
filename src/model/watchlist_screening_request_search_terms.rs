use serde::{Serialize, Deserialize};
///Search inputs for creating a watchlist screening
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningRequestSearchTerms {
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///The numeric or alphanumeric identifier associated with this document. Must be between 4 and 32 characters long, and cannot have leading or trailing spaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    ///The legal name of the individual being screened. Must have at least one alphabetical character, have a maximum length of 100 characters, and not include leading or trailing spaces.
    pub legal_name: String,
    ///ID of the associated program.
    pub watchlist_program_id: String,
}
impl std::fmt::Display for WatchlistScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
