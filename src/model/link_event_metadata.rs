use serde::{Serialize, Deserialize};
///Metadata about an event that occurred while the user was going through Link
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkEventMetadata {
    ///The account number mask extracted from the user-provided account number. If the user-inputted account number is four digits long, `account_number_mask` is empty. Emitted by `SUBMIT_ACCOUNT_NUMBER`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number_mask: Option<String>,
    ///The name of the selected brand.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    ///The error code that the user encountered. Emitted by `ERROR`, `EXIT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    ///The error message that the user encountered. Emitted by: `ERROR`, `EXIT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    ///The error type that the user encountered. Emitted by: `ERROR`, `EXIT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    ///The status key indicates the point at which the user exited the Link flow. Emitted by: `EXIT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_status: Option<String>,
    ///The ID of the selected institution. Emitted by: all events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The name of the selected institution. Emitted by: all events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    ///The query used to search for institutions. Emitted by: `SEARCH_INSTITUTION`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_search_query: Option<String>,
    ///The reason this institution was matched, which will be either `returning_user` or `routing_number`. Emitted by: `MATCHED_SELECT_INSTITUTION`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_reason: Option<String>,
    ///If set, the user has encountered one of the following MFA types: code, device, questions, selections. Emitted by: `SUBMIT_MFA` and `TRANSITION_VIEW` when `view_name` is `MFA`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_type: Option<String>,
    ///The request ID for the last request made by Link. This can be shared with Plaid Support to expedite investigation. Emitted by: all events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    ///The routing number submitted by user at the micro-deposits routing number pane. Emitted by `SUBMIT_ROUTING_NUMBER`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    ///Either the verification method for a matched institution selected by the user or the Auth Type Select flow type selected by the user. If selection is used to describe selected verification method, then possible values are `phoneotp` or `password`;  if selection is used to describe the selected Auth Type Select flow, then possible values are `flow_type_manual` or `flow_type_instant`. Emitted by: `MATCHED_SELECT_VERIFY_METHOD` and `SELECT_AUTH_TYPE`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection: Option<String>,
    ///The name of the view that is being transitioned to. Emitted by: `TRANSITION_VIEW`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
}
impl std::fmt::Display for LinkEventMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
