use serde::{Serialize, Deserialize};
///Names that are explicitly marked as low quality either by their `source` list, or by `plaid` by a series of additional checks done by Plaid. Plaid does not ever surface a hit as a result of a weak name alone. If a name has no quality issues, this value will be `none`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WeakAliasDetermination {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "plaid")]
    Plaid,
}
