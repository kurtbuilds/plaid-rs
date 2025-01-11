use serde::{Serialize, Deserialize};
///This object includes a code and description to describe medium risk transactions and above on /accounts/balance/get.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskReason {
    /**A code that represents the type of risk associated with the proposed transaction.

The codes are from PL01 to PL08 and from BK01 to BK07. For a full listing of risk reason codes, see [Risk codes](https://plaid.com/docs/balance/balance-plus/#risk-codes).*/
    pub code: String,
    ///A human-readable description explaining the risk code associated with the proposed transaction and some recommended actions. This field is subject to change; any programmatic logic should be based on the `code` field instead.
    pub description: String,
}
impl std::fmt::Display for RiskReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
