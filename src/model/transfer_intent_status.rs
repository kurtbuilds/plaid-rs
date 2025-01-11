use serde::{Serialize, Deserialize};
/**The status of the transfer intent.

`PENDING`: The transfer intent is pending.
`SUCCEEDED`: The transfer intent was successfully created.
`FAILED`: The transfer intent was unable to be created.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferIntentStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "FAILED")]
    Failed,
}
