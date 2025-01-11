use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Base Report product. This field is required if `assets` is included in the `products` array and the client is CRA-enabled.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestBaseReport {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    ///The maximum integer number of days of history to include in the Base Report.
    pub days_requested: i64,
}
impl std::fmt::Display for LinkTokenCreateRequestBaseReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
