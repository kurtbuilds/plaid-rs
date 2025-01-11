use serde::{Serialize, Deserialize};
///CraLoansApplicationsRegisterResponse defines the response schema for `/cra/loans/applications/register`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraLoansApplicationsRegisterResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraLoansApplicationsRegisterResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
