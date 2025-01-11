use serde::{Serialize, Deserialize};
use super::{BalancePlusAttributes, BalancePlusRiskLevel, RiskReason};
///Provides a detailed risk assessment for the requested transaction. For this field to be returned, the client must be enrolled in the Balance Plus beta program and the [`payment_details`](http://plaid.com/docs/balance/balance-plus/#accounts-balance-get-request-payment-details) object must have been sent in the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsBalanceGetResponsePaymentRiskAssessment {
    ///Contains additional data that can be used to assess the ACH return risk
    pub attributes: BalancePlusAttributes,
    ///Timestamp of the last successful balance update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub balance_last_updated: chrono::DateTime<chrono::Utc>,
    /**Whether the proposed transaction exceeds the balance threshold set in the request. `true` indicates higher risk; `false` indicates lower risk. If the `amount` multiplied by the `balance_threshold_percentage` (as a percentage) exceeds the balance in the account, then `exceeds_balance_threshold` will be true, otherwise, it will be false. For example, if the `amount` is 200 and the `balance_threshold_percentage` is 90, then the account balance must be at least 180 for `exceeds_balance_threshold` to be false.

By default, the available balance will be used for this calculation; if it cannot be obtained, the current balance will be used.

This field is particularly useful for customers using indirect Items and who do not have direct access to raw balance data.*/
    pub exceeds_balance_threshold: bool,
    /**A five-tier risk assessment for the transaction, based on the probability distribution of ACH returns, measured by the incident rate.

Each tier corresponds to a distribution with a different mean (average) probability.

`HIGH`: The mean probability of ACH return risk is above 40%.
`MEDIUM_HIGH`: The mean probability of ACH return risk is 15%-40%.
`MEDIUM`: The mean probability of ACH return risk is 5-10%.
`MEDIUM_LOW`: The mean probability of ACH return risk is 1%-2%.
`LOW`: The mean probability of ACH return risk is below 1%.

Note that these tiers correspond to probability *distributions* and not to discrete probabilities.

These tier definitions are specific to Balance Plus and do not apply to risk tiers generated by other Plaid endpoints.*/
    pub risk_level: BalancePlusRiskLevel,
    ///An array of objects, each representing a specific reason contributing to the risk assessment of an ACH transaction. This field is only supplied for transactions classified as `HIGH`, `MEDIUM-HIGH`, or `MEDIUM` risk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_reasons: Option<Vec<RiskReason>>,
    /**A risk score ranging from 1-99, reflecting the likelihood of ACH debit return.
A higher score indicates a greater risk of return, often due to overdrawn accounts or account
ineligibility to receive ACH transactions. Typical return codes include "R01", "R02", "R03",
"R04", "R06", "R08", "R09", "R13", "R16", "R17", "R20", "R23", etc., with a turnaround of 2 banking days.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}
impl std::fmt::Display for AccountsBalanceGetResponsePaymentRiskAssessment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
