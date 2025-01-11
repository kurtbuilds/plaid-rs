use serde::{Serialize, Deserialize};
use super::{LoanPaymentsCounts, LoanPaymentsMerchantCounts};
///An object representing the loan exposure subcategory of the report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringLoanInsights {
    ///The number of loan disbursements detected in the last 30 days
    pub loan_disbursements_count: f64,
    ///Details regarding the number of unique loan payment merchants
    pub loan_payment_merchants_counts: LoanPaymentsMerchantCounts,
    ///Details regarding the number of loan payments
    pub loan_payments_counts: LoanPaymentsCounts,
}
impl std::fmt::Display for MonitoringLoanInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
