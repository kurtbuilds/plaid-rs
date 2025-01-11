use serde::{Serialize, Deserialize};
use super::CreditAmountWithCurrency;
///Average balance in dollar amount per month
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAverageMonthlyBalances {
    ///This contains an amount, denominated in the currency specified by either `iso_currency_code` or `unofficial_currency_code`
    pub average_balance: CreditAmountWithCurrency,
    /**The end date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: String,
    /**The start date of this time period.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: String,
}
impl std::fmt::Display for BaseReportAverageMonthlyBalances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
