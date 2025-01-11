use serde::{Serialize, Deserialize};
///Indicates whether the email domain is listed as disposable if known. Disposable domains are often used to create email addresses that are part of a fake set of user details.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckEmailDomainIsDisposable {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
