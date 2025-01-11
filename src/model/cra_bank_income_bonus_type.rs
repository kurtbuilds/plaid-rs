use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeBonusType(pub serde_json::Value);
