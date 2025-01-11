use serde::{Serialize, Deserialize};
///The account type. This will always be `depository`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditBankIncomeAccountType {
    #[serde(rename = "depository")]
    Depository,
}
