use serde::{Serialize, Deserialize};
///Indicates whether the email address domain is custom if known, i.e. a company domain and not free or disposable.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckEmailDomainIsCustom {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
