use serde::{Serialize, Deserialize};
///To enable Balance Plus (beta), send this object as part of the `/accounts/balance/get` request. Only available to customers participating in the Balance Plus beta program; to enroll in the beta, contact your account manager. If this object is present in the request, the [`payment_risk_assessment`](https://plaid.com/docs/balance/balance-plus/#accounts-balance-get-response-payment-risk-assessment-risk-level) object will be present in the response.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountsBalanceGetRequestPaymentDetails {
    /**The Plaid `account_id` of the account that is the funding source for the proposed transaction. The `account_id` is returned in the `/accounts/get` endpoint as well as the [`onSuccess`](/docs/link/ios/#link-ios-onsuccess-linkSuccess-metadata-accounts-id) callback metadata.

This will return an [`INVALID_ACCOUNT_ID`](/docs/errors/invalid-input/#invalid_account_id) error if the account has been removed at the bank or if the `account_id` is no longer valid.*/
    pub account_id: String,
    ///The transaction amount, in USD (e.g. `102.05`)
    pub amount: f64,
    /**If the `amount` multiplied by the `balance_threshold_percentage` (as a percentage) exceeds the balance in the account, then [`payment_risk_assessment.exceeds_balance_threshold`](https://plaid.com/docs/balance/balance-plus/#accounts-balance-get-response-payment-risk-assessment-exceeds-balance-threshold) in the response will be true, otherwise, it will be false. For example, if the `amount` is 200 and the `balance_threshold_percentage` is 90, then the account balance must be at least 180 for `exceeds_balance_threshold` to be false.

By default, the available balance will be used for this calculation; if it cannot be obtained, the current balance will be used.

This field is particularly useful for customers using indirect Items and who do not have direct access to raw balance data.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_threshold_percentage: Option<i64>,
    ///The unique ID that you would like to use to refer to this transaction. For your convenience mapping your internal data, you could use your internal identifier for this transaction.
    pub client_transaction_id: String,
    ///A boolean that determines whether the balance has to be refreshed in real time as part of the API call when using Balance Plus. Setting this to field to `true` will result in more recent balances, but latency may be up to 30 seconds or more. If making a regular (non-Balance Plus) Balance call, without the `payment_details` object present, `/accounts/balance/get` will always return real-time balances.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_real_time_balance_refresh: Option<bool>,
}
impl std::fmt::Display for AccountsBalanceGetRequestPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
