use serde::{Serialize, Deserialize};
///Indicates whether the email address domain is a free provider such as Gmail or Hotmail if known.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskCheckEmailDomainIsFreeProvider {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
