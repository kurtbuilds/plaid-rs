use serde::{Serialize, Deserialize};
///Flow types to retrieve payroll income data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncomeVerificationPayrollFlowType {
    #[serde(rename = "payroll_digital_income")]
    PayrollDigitalIncome,
    #[serde(rename = "payroll_document_income")]
    PayrollDocumentIncome,
}
