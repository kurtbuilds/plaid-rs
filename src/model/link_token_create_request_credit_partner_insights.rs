use serde::{Serialize, Deserialize};
use super::PrismProduct;
///Specifies options for initializing Link for use with the Credit Partner Insights product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestCreditPartnerInsights {
    ///The maximum integer number of days of history to compute Credit Partner Insights. Defaults to 180 if not specified
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    ///The specific Prism products to return. If none are passed in, then all products will be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prism_products: Option<Vec<PrismProduct>>,
}
impl std::fmt::Display for LinkTokenCreateRequestCreditPartnerInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
