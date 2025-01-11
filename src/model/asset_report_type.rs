use serde::{Serialize, Deserialize};
///Indicates either a Fast Asset Report, which will contain only current identity and balance information, or a Full Asset Report, which will also contain historical balance information and transaction data.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetReportType {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "FAST")]
    Fast,
}
