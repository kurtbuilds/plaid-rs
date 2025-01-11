use serde::{Serialize, Deserialize};
use super::CreditAccountSubtype;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditAccountSubtypes(pub Vec<CreditAccountSubtype>);
