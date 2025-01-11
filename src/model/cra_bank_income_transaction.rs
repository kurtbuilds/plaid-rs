use serde::{Serialize, Deserialize};
use super::CraBankIncomeBonusType;
///The transactions data for the end user's income source(s).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraBankIncomeTransaction {
    /**The settled value of the transaction, denominated in the transaction's currency as stated in `iso_currency_code` or `unofficial_currency_code`.
Positive values when money moves out of the account; negative values when money moves in.
For example, credit card purchases are positive; credit card payment, direct deposits, and refunds are negative.*/
    pub amount: f64,
    /**The type of bonus that this transaction represents, if it is a bonus.
`BONUS_INCLUDED`: Bonus is included in this transaction along with the normal pay
`BONUS_ONLY`: This transaction is a standalone bonus*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_type: Option<CraBankIncomeBonusType>,
    ///The check number of the transaction. This field is only populated for check transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /**For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted.
Both dates are returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub date: chrono::NaiveDate,
    ///The ISO 4217 currency code of the amount or balance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    ///The merchant name or transaction description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The string returned by the financial institution to describe the transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_description: Option<String>,
    /**When true, identifies the transaction as pending or unsettled.
Pending transaction details (name, type, amount, category ID) may change before they are settled.*/
    pub pending: bool,
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: String,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CraBankIncomeTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
