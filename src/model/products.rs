use serde::{Serialize, Deserialize};
///A list of products that an institution can support. All Items must be initialized with at least one product. The Balance product is always available and does not need to be specified during initialization.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Products {
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "balance")]
    Balance,
    #[serde(rename = "balance_plus")]
    BalancePlus,
    #[serde(rename = "beacon")]
    Beacon,
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "identity_match")]
    IdentityMatch,
    #[serde(rename = "investments")]
    Investments,
    #[serde(rename = "investments_auth")]
    InvestmentsAuth,
    #[serde(rename = "liabilities")]
    Liabilities,
    #[serde(rename = "payment_initiation")]
    PaymentInitiation,
    #[serde(rename = "identity_verification")]
    IdentityVerification,
    #[serde(rename = "transactions")]
    Transactions,
    #[serde(rename = "transactions_refresh")]
    TransactionsRefresh,
    #[serde(rename = "credit_details")]
    CreditDetails,
    #[serde(rename = "income")]
    Income,
    #[serde(rename = "income_verification")]
    IncomeVerification,
    #[serde(rename = "standing_orders")]
    StandingOrders,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "employment")]
    Employment,
    #[serde(rename = "recurring_transactions")]
    RecurringTransactions,
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "statements")]
    Statements,
    #[serde(rename = "processor_payments")]
    ProcessorPayments,
    #[serde(rename = "processor_identity")]
    ProcessorIdentity,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "cra_base_report")]
    CraBaseReport,
    #[serde(rename = "cra_income_insights")]
    CraIncomeInsights,
    #[serde(rename = "cra_partner_insights")]
    CraPartnerInsights,
    #[serde(rename = "cra_network_insights")]
    CraNetworkInsights,
    #[serde(rename = "cra_cashflow_insights")]
    CraCashflowInsights,
    #[serde(rename = "layer")]
    Layer,
    #[serde(rename = "pay_by_bank")]
    PayByBank,
}
