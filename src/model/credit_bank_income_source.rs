use serde::{Serialize, Deserialize};
use super::{
    CreditBankIncomeCategory, CreditBankIncomeHistoricalSummary,
    CreditBankIncomePayFrequency,
};
///Detailed information for the income source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeSource {
    ///Plaid's unique identifier for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
    /**The income category.
`BANK_INTEREST`: Interest earned from a bank account.
`BENEFIT_OTHER`: Government benefits other than retirement, unemployment, child support, or disability. Currently used only in the UK, to represent benefits such as Cost of Living Payments.
`CASH`: Deprecated and used only for existing legacy implementations. Has been replaced by `CASH_DEPOSIT` and `TRANSFER_FROM_APPLICATION`.
`CASH_DEPOSIT`: A cash or check deposit.
`CHILD_SUPPORT`: Child support payments received.
`GIG_ECONOMY`: Income earned as a gig economy worker, e.g. driving for Uber, Lyft, Postmates, DoorDash, etc.
`LONG_TERM_DISABILITY`: Disability payments, including Social Security disability benefits.
`OTHER`: Income that could not be categorized as any other income category.
`MILITARY`: Veterans benefits. Income earned as salary for serving in the military (e.g. through DFAS) will be classified as `SALARY` rather than `MILITARY`.
`RENTAL`: Income earned from a rental property. Income may be identified as rental when the payment is received through a rental platform, e.g. Airbnb; rent paid directly by the tenant to the property owner (e.g. via cash, check, or ACH) will typically not be classified as rental income.
`RETIREMENT`: Payments from private retirement systems, pensions, and government retirement programs, including Social Security retirement benefits.
`SALARY`: Payment from an employer to an earner or other form of permanent employment.
`TAX_REFUND`: A tax refund.
`TRANSFER_FROM_APPLICATION`: Deposits from a money transfer app, such as Venmo, Cash App, or Zelle.
`UNEMPLOYMENT`: Unemployment benefits. In the UK, includes certain low-income benefits such as the Universal Credit.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_category: Option<CreditBankIncomeCategory>,
    ///The most common name or original description for the underlying income transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_description: Option<String>,
    ///A unique identifier for an income source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_source_id: Option<String>,
    ///The income pay frequency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<CreditBankIncomePayFrequency>,
    /**Minimum of all dates within the specific income sources in the user's bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    ///Total amount of earnings in the user’s bank account for the specific income source for days requested by the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    ///Number of transactions for the income source within the start and end date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i64>,
}
impl std::fmt::Display for CreditBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
