use serde::{Serialize, Deserialize};
use super::{CraBankIncomeCause, CraBankIncomeWarningCode, CreditBankIncomeWarningType};
///The warning associated with the data that was unavailable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraBankIncomeWarning {
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<CraBankIncomeCause>,
    /**The warning code identifies a specific kind of warning.
`IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item
`TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item
`REPORT_DELETED`: Report deleted due to customer or consumer request
`DATA_UNAVAILABLE`: No relevant data was found for the Item*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_code: Option<CraBankIncomeWarningCode>,
    ///The warning type which will always be `BANK_INCOME_WARNING`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_type: Option<CreditBankIncomeWarningType>,
}
impl std::fmt::Display for CraBankIncomeWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
