use serde::{Serialize, Deserialize};
///HTTP Method to use for the request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxHateoasLinkAction {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PUT")]
    Put,
}
