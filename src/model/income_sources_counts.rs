use serde::{Serialize, Deserialize};
///Details about the number of income sources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncomeSourcesCounts {
    ///The number of income sources detected at the subscription date
    pub baseline_count: f64,
    ///The number of income sources currently detected
    pub current_count: f64,
}
impl std::fmt::Display for IncomeSourcesCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
