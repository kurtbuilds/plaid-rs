use serde::{Serialize, Deserialize};
/**The status of the wallet.

`UNKNOWN`: The wallet status is unknown.

`ACTIVE`: The wallet is active and ready to send money to and receive money from.

`CLOSED`: The wallet is closed. Any transactions made to or from this wallet will error.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WalletStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "CLOSED")]
    Closed,
}
