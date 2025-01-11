use serde::{Serialize, Deserialize};
/**The status of this Identity Verification attempt.


`active` - The Identity Verification attempt is incomplete. The user may have completed part of the session, but has neither failed or passed.

`success` - The Identity Verification attempt has completed, passing all steps defined to the associated Identity Verification template

`failed` - The user failed one or more steps in the session and was told to contact support.

`expired` - The Identity Verification attempt was active for a long period of time without being completed and was automatically marked as expired. Note that sessions currently do not expire. Automatic expiration is expected to be enabled in the future.

`canceled` - The Identity Verification attempt was canceled, either via the dashboard by a user, or via API. The user may have completed part of the session, but has neither failed or passed.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IdentityVerificationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "pending_review")]
    PendingReview,
}
