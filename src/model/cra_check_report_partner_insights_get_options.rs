use serde::{Serialize, Deserialize};
use super::{PrismProduct, PrismVersions};
///Defines configuration options to generate Partner Insights
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraCheckReportPartnerInsightsGetOptions {
    ///The specific Prism Data products to return. If none are passed in, then all products will be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prism_products: Option<Vec<PrismProduct>>,
    ///The versions of Prism products to evaluate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prism_versions: Option<PrismVersions>,
}
impl std::fmt::Display for CraCheckReportPartnerInsightsGetOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
