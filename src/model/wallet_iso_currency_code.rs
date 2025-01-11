use serde::{Serialize, Deserialize};
///An ISO-4217 currency code, used with e-wallets and transactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WalletIsoCurrencyCode {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
}
