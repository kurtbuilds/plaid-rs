use serde::{Serialize, Deserialize};
/**Indicator of account number format validity for institution.

`valid`: indicates that the account number has a correct format for the institution.

`invalid`: indicates that the account number has an incorrect format for the institution.

`unknown`: indicates that there was not enough information to determine whether the format is correct for the institution.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountVerificationInsightsAccountNumberFormat {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "unknown")]
    Unknown,
}
