use serde::{Serialize, Deserialize};
///CraLoansUpdateResponse defines the response schema for `/cra/loans/update`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraLoansUpdateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraLoansUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
