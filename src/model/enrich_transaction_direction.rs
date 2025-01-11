use serde::{Serialize, Deserialize};
/**The direction of the transaction from the perspective of the account holder:

`OUTFLOW` - Includes outgoing transfers, purchases, and fees. (Typically represented as a negative value on checking accounts and debit cards and a positive value on credit cards.)

`INFLOW` - Includes incoming transfers, refunds, and income. (Typically represented as a positive value on checking accounts and debit cards and a negative value on credit cards.)*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnrichTransactionDirection {
    #[serde(rename = "INFLOW")]
    Inflow,
    #[serde(rename = "OUTFLOW")]
    Outflow,
}
