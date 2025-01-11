use serde::{Serialize, Deserialize};
/**The status of the given Hosted Link session.

`CREATED`: The session is created but not yet accessed by the user

`OPENED`: The session is opened by the user but not yet completed

`EXITED`: The session has been exited by the user

`COMPLETED`: The session has been completed by the user

`EXPIRED`: The session has expired*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkDeliverySessionStatus {
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "OPENED")]
    Opened,
    #[serde(rename = "EXITED")]
    Exited,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "EXPIRED")]
    Expired,
}
