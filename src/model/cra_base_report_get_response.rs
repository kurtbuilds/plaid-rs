use serde::{Serialize, Deserialize};
use super::{BaseReport, BaseReportWarning};
///CraBaseReportGetResponse defines the response schema for `/cra/base_report/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBaseReportGetResponse {
    ///An object representing a Base Report
    pub report: BaseReport,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///If the Base Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<BaseReportWarning>,
}
impl std::fmt::Display for CraBaseReportGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
