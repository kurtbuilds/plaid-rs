use serde::{Serialize, Deserialize};
/**The asynchronous event to be simulated. May be: `posted`, `settled`, `failed`, or `returned`.

An error will be returned if the event type is incompatible with the current ledger sweep status. Compatible status --> event type transitions include:

`sweep.pending` --> `sweep.posted`

`sweep.pending` --> `sweep.failed`

`sweep.posted` --> `sweep.settled`

`sweep.posted` --> `sweep.returned`

`sweep.settled` --> `sweep.returned`*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferLedgerSweepSimulateEventType {
    #[serde(rename = "sweep.posted")]
    SweepPosted,
    #[serde(rename = "sweep.settled")]
    SweepSettled,
    #[serde(rename = "sweep.returned")]
    SweepReturned,
    #[serde(rename = "sweep.failed")]
    SweepFailed,
}
