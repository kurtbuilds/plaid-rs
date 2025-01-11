use serde::{Serialize, Deserialize};
use super::OtherAccountSubtype;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OtherAccountSubtypes(pub Vec<OtherAccountSubtype>);
