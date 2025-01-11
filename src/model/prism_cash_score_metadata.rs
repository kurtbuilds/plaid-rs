use serde::{Serialize, Deserialize};
///An object containing metadata about the provided transactions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrismCashScoreMetadata {
    ///Number of credit transactions in the last 30 days.
    #[serde(rename = "l1m_credit_value_cnt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l1_m_credit_value_cnt: Option<i64>,
    ///Number of debit transactions in the last 30 days.
    #[serde(rename = "l1m_debit_value_cnt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l1_m_debit_value_cnt: Option<i64>,
    ///Number of days since the oldest transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
    ///Number of days since the oldest credit transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age_credit: Option<i64>,
    ///Number of days since the oldest debit transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age_debit: Option<i64>,
    ///Number of days since the latest transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_age: Option<i64>,
    ///Number of days since the latest credit transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_age_credit: Option<i64>,
    ///Number of days since the latest debit transaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_age_debit: Option<i64>,
    ///Number of credit transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_trxn_credit: Option<i64>,
    ///Number of debit transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_trxn_debit: Option<i64>,
}
impl std::fmt::Display for PrismCashScoreMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
