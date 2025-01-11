use serde::{Serialize, Deserialize};
/**The result of the bank income refresh report generation

`SUCCESS`: The refreshed report was successfully generated and can be retrieved via `/credit/bank_income/get`.

`FAILURE`: The refreshed report failed to be generated*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BankIncomeRefreshCompleteResult {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}
