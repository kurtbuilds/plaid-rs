use serde::{Serialize, Deserialize};
///A value from a MISMO prescribed list that specifies financial assets in a mortgage loan transaction. Assets may be either liquid or fixed and are associated with a corresponding asset amount.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssetType {
    CheckingAccount,
    SavingsAccount,
    Investment,
    MoneyMarketFund,
    Other,
}
