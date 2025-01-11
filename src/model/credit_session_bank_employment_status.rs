use serde::{Serialize, Deserialize};
/**Status of the Bank Employment Link session.

`APPROVED`: User has approved and verified their employment.

`NO_EMPLOYMENTS_FOUND`: We attempted, but were unable to find any employment in the connected account.

`EMPLOYER_NOT_LISTED`: The user explicitly indicated that they did not see their current or previous employer in the list of employer names found.

`STARTED`: The user began the bank income portion of the link flow.

`INTERNAL_ERROR`: The user encountered an internal error.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditSessionBankEmploymentStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "NO_EMPLOYERS_FOUND")]
    NoEmployersFound,
    #[serde(rename = "EMPLOYER_NOT_LISTED")]
    EmployerNotListed,
}
