
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionStreamAmount {
    pub amount: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for TransactionStreamAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}