use serde::{Serialize, Deserialize};
use super::CraNetworkInsightsReport;
///CraCheckReportNetworkInsightsGetResponse defines the response schema for `/cra/check_report/network_attributes/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraCheckReportNetworkInsightsGetResponse {
    ///Contains data for the CRA Network Attributes Report.
    pub report: CraNetworkInsightsReport,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraCheckReportNetworkInsightsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
