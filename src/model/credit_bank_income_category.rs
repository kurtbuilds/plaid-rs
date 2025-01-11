use serde::{Serialize, Deserialize};
/**The income category.
`BANK_INTEREST`: Interest earned from a bank account.
`BENEFIT_OTHER`: Government benefits other than retirement, unemployment, child support, or disability. Currently used only in the UK, to represent benefits such as Cost of Living Payments.
`CASH`: Deprecated and used only for existing legacy implementations. Has been replaced by `CASH_DEPOSIT` and `TRANSFER_FROM_APPLICATION`.
`CASH_DEPOSIT`: A cash or check deposit.
`CHILD_SUPPORT`: Child support payments received.
`GIG_ECONOMY`: Income earned as a gig economy worker, e.g. driving for Uber, Lyft, Postmates, DoorDash, etc.
`LONG_TERM_DISABILITY`: Disability payments, including Social Security disability benefits.
`OTHER`: Income that could not be categorized as any other income category.
`MILITARY`: Veterans benefits. Income earned as salary for serving in the military (e.g. through DFAS) will be classified as `SALARY` rather than `MILITARY`.
`RENTAL`: Income earned from a rental property. Income may be identified as rental when the payment is received through a rental platform, e.g. Airbnb; rent paid directly by the tenant to the property owner (e.g. via cash, check, or ACH) will typically not be classified as rental income.
`RETIREMENT`: Payments from private retirement systems, pensions, and government retirement programs, including Social Security retirement benefits.
`SALARY`: Payment from an employer to an earner or other form of permanent employment.
`TAX_REFUND`: A tax refund.
`TRANSFER_FROM_APPLICATION`: Deposits from a money transfer app, such as Venmo, Cash App, or Zelle.
`UNEMPLOYMENT`: Unemployment benefits. In the UK, includes certain low-income benefits such as the Universal Credit.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditBankIncomeCategory {
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "UNEMPLOYMENT")]
    Unemployment,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "GIG_ECONOMY")]
    GigEconomy,
    #[serde(rename = "RENTAL")]
    Rental,
    #[serde(rename = "CHILD_SUPPORT")]
    ChildSupport,
    #[serde(rename = "MILITARY")]
    Military,
    #[serde(rename = "RETIREMENT")]
    Retirement,
    #[serde(rename = "LONG_TERM_DISABILITY")]
    LongTermDisability,
    #[serde(rename = "BANK_INTEREST")]
    BankInterest,
    #[serde(rename = "CASH_DEPOSIT")]
    CashDeposit,
    #[serde(rename = "TRANSFER_FROM_APPLICATION")]
    TransferFromApplication,
    #[serde(rename = "TAX_REFUND")]
    TaxRefund,
    #[serde(rename = "BENEFIT_OTHER")]
    BenefitOther,
    #[serde(rename = "OTHER")]
    Other,
}
