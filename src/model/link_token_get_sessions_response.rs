use serde::{Serialize, Deserialize};
use super::{
    LinkEvent, LinkSessionExit, LinkSessionExitDeprecated, LinkSessionResults,
    LinkSessionSuccess,
};
///An object containing information about a link session. Session data will be provided for up to six hours after the session has ended.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenGetSessionsResponse {
    ///List of customer-related Link events
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<LinkEvent>>,
    ///An object representing an [onExit](https://plaid.com/docs/link/web/#onexit) callback from Link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<LinkSessionExit>,
    ///The timestamp at which the link session was finished, if available, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The unique ID for the link session.
    pub link_session_id: String,
    ///An object representing an [onExit](https://plaid.com/docs/link/web/#onexit) callback from Link. This field has been deprecated in favor of [`exit`](https://plaid.com/docs/api/link/#link-token-get-response-link-sessions-exit), for improved naming consistency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<LinkSessionExitDeprecated>,
    ///An object representing an [onSuccess](https://plaid.com/docs/link/web/#onsuccess) callback from Link. This object has been deprecated in favor of the newer [`results.item_add_result`](https://plaid.com/docs/api/link/#link-token-get-response-link-sessions-results-item-add-results), which can support multiple public tokens in a single Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_success: Option<LinkSessionSuccess>,
    ///The set of results for a Link session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<LinkSessionResults>,
    ///The timestamp at which the link session was first started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for LinkTokenGetSessionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
