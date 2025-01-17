use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for [update mode](https://plaid.com/docs/link/update-mode).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUpdate {
    ///If `true`, enables [update mode with Account Select](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts) for institutions in the US and Canada that do not use OAuth, or that use OAuth but do not have their own account selection flow. For institutions in the US that have an OAuth account selection flow (i.e. most OAuth-enabled institutions), update mode with Account Select will always be enabled, regardless of the value of this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_selection_enabled: Option<bool>,
    ///An array of `item_id`s associated with the user to be updated in update mode. If empty or `null`, this field will default to initializing update mode for the most recent unhealthy Item associated with the user. A `user_token` must also be provided to use this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    ///By default, Plaid will enable the reauthorization flow during update mode for an Item enabled for [Data Transparency Messaging](https://plaid.com/docs/link/data-transparency-messaging-migration-guide/) if the Item expires within six months. During a reauthorization flow, an end user will review Plaid's end user privacy policy, use case and data scope consents, and account access consents; they may also be required to log in to their financial institution's OAuth flow. After the end user successfully completes the reauthorization flow, the Item's expiration date will be extended to 12 months from the time that the reauthorization took place. This field allows you to optionally override the default reauthorization scheduling logic to either forcibly enable or disable the reauthorization flow for a given update mode session. This field does not impact the flow for Items at institutions in the EU or UK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reauthorization_enabled: Option<bool>,
    ///If `true`, a `user_token` must also be provided, and Link will open in update mode for the given user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
