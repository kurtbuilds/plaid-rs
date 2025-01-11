use serde::{Serialize, Deserialize};
use super::DepositoryAccountSubtype;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositoryAccountSubtypes(pub Vec<DepositoryAccountSubtype>);
