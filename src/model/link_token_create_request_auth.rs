use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Auth product. This field can be used to enable or disable extended Auth flows for the resulting Link session. Omitting any field will result in a default that can be configured by your account manager. The default behavior described in the documentation is the default behavior that will apply if you have not requested your account manager to apply a different default.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestAuth {
    ///Specifies whether Auth Type Select is enabled for the Link session, allowing the end user to choose between linking via a credentials-based flow (i.e. Instant Auth, Instant Match, Automated Micro-deposits) or a manual flow that does not require login (all other Auth flows) prior to selecting their financial institution. Default behavior is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type_select_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Automated Micro-deposits flow. Default behavior is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated_microdeposits_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Database Insights flow. Database Insights is currently in closed beta; for access, contact your Account Manager. Default behavior is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_insights_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Database Match flow. Default behavior is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_match_enabled: Option<bool>,
    ///This field has been deprecated in favor of `auth_type_select_enabled`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<String>,
    ///Specifies whether the Link session is enabled for the Instant Match flow. Instant Match is enabled by default. Instant Match can be disabled by setting this field to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instant_match_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for the Instant Micro-deposits flow.  Default behavior for Plaid teams created after November 2023 is `false`; default behavior for Plaid teams created before that date is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instant_microdeposits_enabled: Option<bool>,
    ///Specifies what type of [Reroute to Credentials](https://plaid.com/docs/auth/coverage/same-day/#reroute-to-credentials) pane should be used in the Link session for the Same Day Micro-deposits flow. Default behavior is `OPTIONAL`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reroute_to_credentials: Option<String>,
    ///Specifies whether the Link session is enabled for the Same Day Micro-deposits flow.  Default behavior is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub same_day_microdeposits_enabled: Option<bool>,
    ///Specifies whether the Link session is enabled for SMS micro-deposits verification. Default behavior is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_microdeposits_verification_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
