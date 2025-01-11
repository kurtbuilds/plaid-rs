use serde::{Serialize, Deserialize};
/**`digital:` transactions that took place online.

`place:` transactions that were made at a physical location.

`special:` transactions that relate to banks, e.g. fees or deposits.

`unresolved:` transactions that do not fit into the other three types.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetReportTransactionType {
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "place")]
    Place,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "unresolved")]
    Unresolved,
}
