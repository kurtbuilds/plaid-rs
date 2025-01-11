use serde::{Serialize, Deserialize};
///Valid account subtypes for depository accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-depository).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DepositoryAccountSubtype {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "money market")]
    MoneyMarket,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "cash management")]
    CashManagement,
    #[serde(rename = "ebt")]
    Ebt,
    #[serde(rename = "all")]
    All,
}
