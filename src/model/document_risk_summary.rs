use serde::{Serialize, Deserialize};
///A summary across all risk signals associated with a document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRiskSummary {
    ///A number between 0 and 100, inclusive, where a score closer to 0 indicates a document is likely to be trustworthy and a score closer to 100 indicates a document is likely to be fraudulent. You can automatically reject documents with a high risk score, automatically accept documents with a low risk score, and manually review documents in between. We suggest starting with a threshold of 80 for auto-rejection and 20 for auto-acceptance. As you gather more data points on typical risk scores for your use case, you can tune these parameters to reduce the number of documents undergoing manual review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<f64>,
}
impl std::fmt::Display for DocumentRiskSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
