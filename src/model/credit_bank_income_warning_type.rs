use serde::{Serialize, Deserialize};
///The warning type which will always be `BANK_INCOME_WARNING`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditBankIncomeWarningType {
    #[serde(rename = "BANK_INCOME_WARNING")]
    BankIncomeWarning,
}
