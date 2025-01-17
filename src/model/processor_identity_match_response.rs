use serde::{Serialize, Deserialize};
use super::AccountIdentityMatchScore;
///ProcessorIdentityMatchResponse defines the response schema for `/processor/identity/match`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorIdentityMatchResponse {
    ///Identity match scores for an account
    pub account: AccountIdentityMatchScore,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorIdentityMatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
