use serde::{Serialize, Deserialize};
///Indicates whether the email address top level domain, which is the last part of the domain, is fraudulent or risky if known. In most cases, a suspicious top level domain is also associated with a disposable or high-risk domain.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckEmailTopLevelDomainIsSuspicious {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
