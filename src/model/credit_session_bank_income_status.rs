use serde::{Serialize, Deserialize};
/**Status of the Bank Income Link session.

`APPROVED`: User has approved and verified their income

`NO_DEPOSITS_FOUND`: We attempted, but were unable to find any income in the connected account.

`USER_REPORTED_NO_INCOME`: The user explicitly indicated that they don't receive income in the connected account.

`STARTED`: The user began the bank income portion of the link flow.

`INTERNAL_ERROR`: The user encountered an internal error.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditSessionBankIncomeStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "NO_DEPOSITS_FOUND")]
    NoDepositsFound,
    #[serde(rename = "USER_REPORTED_NO_INCOME")]
    UserReportedNoIncome,
}
