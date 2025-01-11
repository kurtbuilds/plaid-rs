use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionSuccessMetadataTransferStatus(pub serde_json::Value);
