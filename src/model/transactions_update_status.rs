use serde::{Serialize, Deserialize};
/**A description of the update status for transaction pulls of an Item.

`TRANSACTIONS_UPDATE_STATUS_UNKNOWN`: Unable to fetch transactions update status for Item.
`NOT_READY`: The Item is pending transaction pull.
`INITIAL_UPDATE_COMPLETE`: Initial pull for the Item is complete, historical pull is pending.
`HISTORICAL_UPDATE_COMPLETE`: Both initial and historical pull for Item are complete.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionsUpdateStatus {
    #[serde(rename = "TRANSACTIONS_UPDATE_STATUS_UNKNOWN")]
    TransactionsUpdateStatusUnknown,
    #[serde(rename = "NOT_READY")]
    NotReady,
    #[serde(rename = "INITIAL_UPDATE_COMPLETE")]
    InitialUpdateComplete,
    #[serde(rename = "HISTORICAL_UPDATE_COMPLETE")]
    HistoricalUpdateComplete,
}
