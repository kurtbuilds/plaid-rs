use serde::{Serialize, Deserialize};
use super::CraPartnerInsights;
///CraPartnerInsightsGetResponse defines the response schema for `/cra/partner_insights/get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraCheckReportPartnerInsightsGetResponse {
    ///The Partner Insights report of the bank data for an end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<CraPartnerInsights>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraCheckReportPartnerInsightsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
