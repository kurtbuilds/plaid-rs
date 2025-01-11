use serde::{Serialize, Deserialize};
/**A status of a Beacon User.

`rejected`: The Beacon User has been rejected for fraud. Users can be automatically or manually rejected.

`pending_review`: The Beacon User has been marked for review.

`cleared`: The Beacon User has been cleared of fraud.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BeaconUserStatus {
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "cleared")]
    Cleared,
}
