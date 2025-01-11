use serde::{Serialize, Deserialize};
///A value from a MISMO prescribed list that specifies the type of loan identifier.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoanIdentifierType {
    LenderLoan,
    UniversalLoan,
}
