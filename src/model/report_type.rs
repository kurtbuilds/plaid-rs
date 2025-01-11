use serde::{Serialize, Deserialize};
///The report type. It can be `asset`. Income report types are not yet supported.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReportType {
    #[serde(rename = "asset")]
    Asset,
}
