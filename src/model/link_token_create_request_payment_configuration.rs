use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Pay By Bank flow. This is an optional field to configure the user experience, and currently requires the amount field to be set.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestPaymentConfiguration {
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    ///The description of the transfer that provides the payment context. The max length is 256.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestPaymentConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
