use serde::{Serialize, Deserialize};
///CraBankIncomeCreateRequest defines the response schema for `/cra/bank_income/create`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for CraBankIncomeCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
