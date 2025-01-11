use serde::{Serialize, Deserialize};
///CraBaseReportCreateResponse defines the response schema for `cra/base_report/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBaseReportCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraBaseReportCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
