use serde::{Serialize, Deserialize};
/**An instruction specifying what steps the new Identity Verification attempt should require the user to complete:


`reset` - Restart the user at the beginning of the session, regardless of whether they successfully completed part of their previous session.

`incomplete` - Start the new session at the step that the user failed in the previous session, skipping steps that have already been successfully completed.

`infer` - If the most recent Identity Verification attempt associated with the given `client_user_id` has a status of `failed` or `expired`, retry using the `incomplete` strategy. Otherwise, use the `reset` strategy.

`custom` - Start the new session with a custom configuration, specified by the value of the `steps` field

Note:

The `incomplete` strategy cannot be applied if the session's failing step is `screening` or `risk_check`.

The `infer` strategy cannot be applied if the session's status is still `active`*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Strategy {
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "infer")]
    Infer,
    #[serde(rename = "custom")]
    Custom,
}
