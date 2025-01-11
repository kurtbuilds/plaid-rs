use serde::{Serialize, Deserialize};
///An ISO-4217 currency code.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IsoCurrencyCode {
    #[serde(rename = "USD")]
    Usd,
}
