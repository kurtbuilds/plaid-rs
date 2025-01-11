use serde::{Serialize, Deserialize};
///The ISO-4217 currency code of the payment. For standing orders and payment consents, `"GBP"` must be used. For Poland, Denmark, Sweden and Norway, only the local currency is currently supported.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentAmountCurrency {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "NOK")]
    Nok,
}
