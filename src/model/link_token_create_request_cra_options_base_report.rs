use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Base Report product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestCraOptionsBaseReport {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestCraOptionsBaseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
