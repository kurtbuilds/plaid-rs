
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEntityScreeningRequestSearchTerms {
    pub country: Option<String>,
    pub document_number: Option<String>,
    pub email_address: Option<String>,
    pub entity_watchlist_program_id: String,
    pub legal_name: Option<String>,
    pub phone_number: Option<String>,
    pub url: Option<String>,
}
impl std::fmt::Display for UpdateEntityScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}