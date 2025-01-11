use serde::{Serialize, Deserialize};
/**The trigger of the sweep

`"manual"` - The sweep is created manually by the customer
`"incoming"` - The sweep is created by incoming funds flow (e.g. Incoming Wire)
`"balance_threshold"` - The sweep is created by balance threshold setting
`"automatic_aggregate"` - The sweep is created by the Plaid automatic aggregation process. These funds did not pass through the Plaid Ledger balance.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SweepTrigger {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "balance_threshold")]
    BalanceThreshold,
    #[serde(rename = "automatic_aggregate")]
    AutomaticAggregate,
}
