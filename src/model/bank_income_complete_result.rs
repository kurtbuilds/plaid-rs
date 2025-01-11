use serde::{Serialize, Deserialize};
/**The result of the bank income report generation

`SUCCESS`: The bank income report was successfully generated and can be retrieved via `/credit/bank_income/get`.

`FAILURE`: The bank income report failed to be generated*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BankIncomeCompleteResult {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}
