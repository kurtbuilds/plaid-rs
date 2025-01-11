use serde::{Serialize, Deserialize};
use super::InvestmentAccountSubtype;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvestmentAccountSubtypes(pub Vec<InvestmentAccountSubtype>);
