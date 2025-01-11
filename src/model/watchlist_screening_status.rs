use serde::{Serialize, Deserialize};
///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WatchlistScreeningStatus {
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "cleared")]
    Cleared,
}
