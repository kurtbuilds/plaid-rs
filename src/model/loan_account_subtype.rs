use serde::{Serialize, Deserialize};
///Valid account subtypes for loan accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-loan).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoanAccountSubtype {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "home equity")]
    HomeEquity,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "line of credit")]
    LineOfCredit,
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "all")]
    All,
}
