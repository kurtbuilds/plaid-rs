use serde::{Serialize, Deserialize};
///LinkProfileEligibilityCheckResponse defines the response schema for `/link/profile/eligibility/check`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkProfileEligibilityCheckResponse {
    ///Indicates whether Plaid has a profile matching the customer's eligibility requirements for this user
    pub profile_matches: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkProfileEligibilityCheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
