use serde::{Serialize, Deserialize};
use super::{
    IncomeBreakdown, PaystubOverrideDeductions, PaystubOverrideEarnings,
    PaystubOverrideEmployee, PaystubOverrideEmployer, PaystubOverrideNetPay,
    PaystubOverridePayPeriodDetails,
};
///An object representing data from a paystub.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverride {
    ///An object with the deduction information found on a paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deductions: Option<PaystubOverrideDeductions>,
    ///An object representing both a breakdown of earnings on a paystub and the total earnings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earnings: Option<PaystubOverrideEarnings>,
    ///The employee on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<PaystubOverrideEmployee>,
    ///The employer on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer: Option<PaystubOverrideEmployer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    ///An object representing information about the net pay amount on the paystub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_pay: Option<PaystubOverrideNetPay>,
    ///Details about the pay period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_details: Option<PaystubOverridePayPeriodDetails>,
}
impl std::fmt::Display for PaystubOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
