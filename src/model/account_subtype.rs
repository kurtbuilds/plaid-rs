use serde::{Serialize, Deserialize};
///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountSubtype {
    #[serde(rename = "401a")]
    AccountSubtype401A,
    #[serde(rename = "401k")]
    AccountSubtype401K,
    #[serde(rename = "403B")]
    AccountSubtype403B,
    #[serde(rename = "457b")]
    AccountSubtype457B,
    #[serde(rename = "529")]
    AccountSubtype529,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "cash management")]
    CashManagement,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "credit card")]
    CreditCard,
    #[serde(rename = "crypto exchange")]
    CryptoExchange,
    #[serde(rename = "ebt")]
    Ebt,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "home equity")]
    HomeEquity,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "line of credit")]
    LineOfCredit,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "money market")]
    MoneyMarket,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "non-custodial wallet")]
    NonCustodialWallet,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "overdraft")]
    Overdraft,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "payroll")]
    Payroll,
    #[serde(rename = "pension")]
    Pension,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
    #[serde(rename = "rdsp")]
    Rdsp,
    #[serde(rename = "resp")]
    Resp,
    #[serde(rename = "retirement")]
    Retirement,
    #[serde(rename = "rlif")]
    Rlif,
    #[serde(rename = "roth")]
    Roth,
    #[serde(rename = "roth 401k")]
    Roth401K,
    #[serde(rename = "rrif")]
    Rrif,
    #[serde(rename = "rrsp")]
    Rrsp,
    #[serde(rename = "sarsep")]
    Sarsep,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "thrift savings plan")]
    ThriftSavingsPlan,
    #[serde(rename = "tfsa")]
    Tfsa,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "ugma")]
    Ugma,
    #[serde(rename = "utma")]
    Utma,
    #[serde(rename = "variable annuity")]
    VariableAnnuity,
}
