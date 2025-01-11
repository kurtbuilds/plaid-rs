use serde::{Serialize, Deserialize};
///The registry containing the party’s registration with name and id
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxPartyRegistry {
    #[serde(rename = "FDX")]
    Fdx,
    #[serde(rename = "GLEIF")]
    Gleif,
    #[serde(rename = "ICANN")]
    Icann,
    #[serde(rename = "PRIVATE")]
    Private,
}
