
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserDataOverview {
    pub days_available: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_transaction_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
    pub total_inflows: f64,
    pub total_outflows: f64,
    pub transaction_count: i64,
}
impl std::fmt::Display for UserDataOverview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}