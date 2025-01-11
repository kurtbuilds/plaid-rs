use serde::{Serialize, Deserialize};
/**The attributes object contains data that can be used to assess account risk. Examples of data include:
`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconAccountRiskAttributes {
    ///The number of times the account's addresses on file have changed over the past 28 days
    #[serde(rename = "address_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_change_count28_d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 90 days
    #[serde(rename = "address_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_change_count90_d: Option<i64>,
    ///The number of days since the bank account was opened, as reported by the financial institution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_account_opening: Option<i64>,
    ///The number of days since the oldest transaction available to Plaid for this account. This measure, combined with Plaid connection history, can be used to infer the age of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_first_observed_transaction: Option<i64>,
    ///The number of days since the first time the Item was connected to an application via Plaid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_first_plaid_connection: Option<i64>,
    ///The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 30 days
    #[serde(rename = "distinct_ip_addresses_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count30_d: Option<i64>,
    ///The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_ip_addresses_count_3d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count3_d: Option<i64>,
    ///The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_ip_addresses_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count7_d: Option<i64>,
    ///The number of distinct IP addresses linked to the same bank account during Plaid authentication in the last 90 days
    #[serde(rename = "distinct_ip_addresses_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_ip_addresses_count90_d: Option<i64>,
    ///The number of distinct user agents linked to the same bank account during Plaid authentication in the last 30 days
    #[serde(rename = "distinct_user_agents_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count30_d: Option<i64>,
    ///The number of distinct user agents linked to the same bank account during Plaid authentication in the last 3 days
    #[serde(rename = "distinct_user_agents_count_3d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count3_d: Option<i64>,
    ///The number of distinct user agents linked to the same bank account during Plaid authentication in the last 7 days
    #[serde(rename = "distinct_user_agents_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count7_d: Option<i64>,
    ///The number of distinct user agents linked to the same bank account during Plaid authentication in the last 90 days
    #[serde(rename = "distinct_user_agents_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distinct_user_agents_count90_d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 28 days
    #[serde(rename = "email_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_change_count28_d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 90 days
    #[serde(rename = "email_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_change_count90_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count30_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_3d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count3_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count7_d: Option<i64>,
    ///Indicates if the account has been closed by the financial institution or the consumer, or is at risk of being closed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_closed: Option<bool>,
    ///Indicates whether the account has withdrawals and transfers disabled or if access to the account is restricted. This could be due to a freeze by the credit issuer, legal restrictions (e.g., sanctions), or regulatory requirements limiting monthly withdrawals, among other reasons
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_frozen_or_restricted: Option<bool>,
    ///The number of times the account's phone numbers on file have changed over the past 28 days
    #[serde(rename = "phone_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_change_count28_d: Option<i64>,
    ///The number of times the account's phone numbers on file have changed over the past 90 days
    #[serde(rename = "phone_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_change_count90_d: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 30 days
    #[serde(rename = "plaid_connections_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_connections_count30_d: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 7 days
    #[serde(rename = "plaid_connections_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_connections_count7_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count30_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_3d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count3_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count7_d: Option<i64>,
    ///The total number of times the item has been connected to applications via Plaid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_plaid_connections_count: Option<i64>,
}
impl std::fmt::Display for BeaconAccountRiskAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
