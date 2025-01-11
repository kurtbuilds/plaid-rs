use serde::{Serialize, Deserialize};
///The warning type which will always be `BANK_EMPLOYMENT_WARNING`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditBankEmploymentWarningType {
    #[serde(rename = "BANK_EMPLOYMENT_WARNING")]
    BankEmploymentWarning,
}
