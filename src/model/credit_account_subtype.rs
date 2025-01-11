use serde::{Serialize, Deserialize};
///Valid account subtypes for credit accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-credit).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditAccountSubtype {
    #[serde(rename = "credit card")]
    CreditCard,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "all")]
    All,
}
