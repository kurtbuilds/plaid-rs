use serde::{Serialize, Deserialize};
use super::ConsumerDisputeCategory;
///The information about a previously submitted valid dispute statement by the consumer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerDispute {
    ///Type of data being disputed by the consumer
    pub category: ConsumerDisputeCategory,
    ///A unique identifier (UUID) of the consumer dispute that can be used for troubleshooting
    pub consumer_dispute_id: String,
    ///Date of the disputed field (e.g. transaction date), in an ISO 8601 format (YYYY-MM-DD)
    pub dispute_field_create_date: chrono::NaiveDate,
    ///Text content of dispute
    pub statement: String,
}
impl std::fmt::Display for ConsumerDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
