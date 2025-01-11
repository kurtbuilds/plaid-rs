use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrismFirstDetectVersion(pub serde_json::Value);
