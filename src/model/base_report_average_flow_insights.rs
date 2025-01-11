use serde::{Serialize, Deserialize};
use super::CreditAmountWithCurrency;
///Average dollar amount of credit or debit transactions out of the account. This field will only be included for depository accounts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAverageFlowInsights {
    /**The end date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: chrono::NaiveDate,
    /**The start date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: chrono::NaiveDate,
    ///This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`
    pub total_amount: CreditAmountWithCurrency,
}
impl std::fmt::Display for BaseReportAverageFlowInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
