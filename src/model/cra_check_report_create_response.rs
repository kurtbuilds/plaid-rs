use serde::{Serialize, Deserialize};
///CraCheckReportCreateResponse defines the response schema for `/cra/check_report/create`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraCheckReportCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl std::fmt::Display for CraCheckReportCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
