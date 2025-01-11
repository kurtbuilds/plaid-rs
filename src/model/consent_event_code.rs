use serde::{Serialize, Deserialize};
///Codes describing the object of a consent event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConsentEventCode {
    #[serde(rename = "USER_AGREEMENT")]
    UserAgreement,
    #[serde(rename = "USE_CASES")]
    UseCases,
    #[serde(rename = "DATA_SCOPES")]
    DataScopes,
    #[serde(rename = "ACCOUNT_SCOPES")]
    AccountScopes,
}
