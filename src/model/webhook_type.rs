use serde::{Serialize, Deserialize};
///The webhook types that can be fired by this test endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebhookType {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "HOLDINGS")]
    Holdings,
    #[serde(rename = "INVESTMENTS_TRANSACTIONS")]
    InvestmentsTransactions,
    #[serde(rename = "ITEM")]
    Item,
    #[serde(rename = "LIABILITIES")]
    Liabilities,
    #[serde(rename = "TRANSACTIONS")]
    Transactions,
    #[serde(rename = "ASSETS")]
    Assets,
}
