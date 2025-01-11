use serde::{Serialize, Deserialize};
///Type of a document for risk signal analysis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskSignalDocumentType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "BANK_STATEMENT")]
    BankStatement,
    #[serde(rename = "BENEFITS_STATEMENT")]
    BenefitsStatement,
    #[serde(rename = "BUSINESS_FILING")]
    BusinessFiling,
    #[serde(rename = "CHECK")]
    Check,
    #[serde(rename = "DRIVING_LICENSE")]
    DrivingLicense,
    #[serde(rename = "FINANCIAL_STATEMENT")]
    FinancialStatement,
    #[serde(rename = "INVOICE")]
    Invoice,
    #[serde(rename = "PAYSLIP")]
    Payslip,
    #[serde(rename = "SOCIAL_SECURITY_CARD")]
    SocialSecurityCard,
    #[serde(rename = "TAX_FORM")]
    TaxForm,
    #[serde(rename = "UTILITY_BILL")]
    UtilityBill,
}
