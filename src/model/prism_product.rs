use serde::{Serialize, Deserialize};
///The Prism products that can be returned by the Plaid API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrismProduct {
    #[serde(rename = "insights")]
    Insights,
    #[serde(rename = "scores")]
    Scores,
}
