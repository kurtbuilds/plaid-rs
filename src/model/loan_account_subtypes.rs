use serde::{Serialize, Deserialize};
use super::LoanAccountSubtype;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoanAccountSubtypes(pub Vec<LoanAccountSubtype>);
