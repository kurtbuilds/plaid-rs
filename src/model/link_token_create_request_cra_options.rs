use serde::{Serialize, Deserialize};
use super::{
    LinkTokenCreateRequestCraOptionsBaseReport,
    LinkTokenCreateRequestCraOptionsCashflowInsights,
    LinkTokenCreateRequestCraOptionsPartnerInsights,
};
///Specifies options for initializing Link for use with Plaid Check products
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestCraOptions {
    ///Specifies options for initializing Link for use with the Base Report product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_report: Option<LinkTokenCreateRequestCraOptionsBaseReport>,
    ///Specifies options for initializing Link for use with the Cashflow Insights product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cashflow_insights: Option<LinkTokenCreateRequestCraOptionsCashflowInsights>,
    ///The number of days of history to include in Plaid Check products. Default value is 365; maximum is 730; minimum is 180. If a value lower than 180 is provided, a minimum of 180 days of history will be requested.
    pub days_requested: i64,
    ///The minimum number of days of data required for the report to be successfully generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_required: Option<i64>,
    ///Specifies options for initializing Link for use with the Credit Partner Insights product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partner_insights: Option<LinkTokenCreateRequestCraOptionsPartnerInsights>,
}
impl std::fmt::Display for LinkTokenCreateRequestCraOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
