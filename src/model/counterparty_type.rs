use serde::{Serialize, Deserialize};
/**The counterparty type.

`merchant`: a provider of goods or services for purchase
`financial_institution`: a financial entity (bank, credit union, BNPL, fintech)
`payment_app`: a transfer or P2P app (e.g. Zelle)
`marketplace`: a marketplace (e.g DoorDash, Google Play Store)
`payment_terminal`: a point-of-sale payment terminal (e.g Square, Toast)
`income_source`: the payer in an income transaction (e.g., an employer, client, or government agency)*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CounterpartyType {
    #[serde(rename = "merchant")]
    Merchant,
    #[serde(rename = "financial_institution")]
    FinancialInstitution,
    #[serde(rename = "payment_app")]
    PaymentApp,
    #[serde(rename = "marketplace")]
    Marketplace,
    #[serde(rename = "payment_terminal")]
    PaymentTerminal,
    #[serde(rename = "income_source")]
    IncomeSource,
}
