use serde::{Serialize, Deserialize};
///The income pay frequency.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditBankIncomePayFrequency {
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "DAILY")]
    Daily,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
