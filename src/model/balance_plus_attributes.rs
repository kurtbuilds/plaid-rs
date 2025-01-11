use serde::{Serialize, Deserialize};
///Contains additional data that can be used to assess the ACH return risk
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalancePlusAttributes {
    ///The number of times the account's addresses on file have changed over the past 28 days
    #[serde(rename = "address_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_change_count28_d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 90 days
    #[serde(rename = "address_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_change_count90_d: Option<i64>,
    ///The number of days since the first time the Item was connected to an application via Plaid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_first_plaid_connection: Option<i64>,
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
    ///Indicates if the receiver bank account is closed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_closed: Option<bool>,
    ///Indicates if the receiver bank account is either frozen or restricted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_frozen_or_restricted: Option<bool>,
    ///Indicates if the ACH transaction funding account is a savings/money market account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_savings_or_money_market_account: Option<bool>,
    ///The number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited
    #[serde(rename = "nsf_overdraft_transactions_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count30_d: Option<i64>,
    ///The number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited
    #[serde(rename = "nsf_overdraft_transactions_count_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count60_d: Option<i64>,
    ///The number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited
    #[serde(rename = "nsf_overdraft_transactions_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count7_d: Option<i64>,
    ///The number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited
    #[serde(rename = "nsf_overdraft_transactions_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count90_d: Option<i64>,
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
    ///The total number of times the Item has been connected to applications via Plaid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_plaid_connections_count: Option<i64>,
    ///The number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited
    #[serde(rename = "unauthorized_transactions_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count30_d: Option<i64>,
    ///The number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited
    #[serde(rename = "unauthorized_transactions_count_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count60_d: Option<i64>,
    ///The number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited
    #[serde(rename = "unauthorized_transactions_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count7_d: Option<i64>,
    ///The number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited
    #[serde(rename = "unauthorized_transactions_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count90_d: Option<i64>,
}
impl std::fmt::Display for BalancePlusAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
