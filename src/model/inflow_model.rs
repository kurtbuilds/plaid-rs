use serde::{Serialize, Deserialize};
///The `inflow_model` allows you to model a test account that receives regular income or make regular payments on a loan. Any transactions generated by the `inflow_model` will appear in addition to randomly generated test data or transactions specified by `override_accounts`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InflowModel {
    ///Amount of income per month. This value is required if `type` is `monthly-income`.
    pub income_amount: f64,
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the income transaction will appear. The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub payment_day_of_month: f64,
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the balance is calculated for the next payment. The name of the income transaction. This field is required if `type` is `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub statement_day_of_month: String,
    ///The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub transaction_name: String,
    /**Inflow model. One of the following:

`none`: No income

`monthly-income`: Income occurs once per month `monthly-balance-payment`: Pays off the balance on a liability account at the given statement day of month.

`monthly-interest-only-payment`: Makes an interest-only payment on a liability account at the given statement day of month.

Note that account types supported by Liabilities will accrue interest in the Sandbox. The types impacted are account type `credit` with subtype `credit` or `paypal`, and account type `loan` with subtype `student` or `mortgage`.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for InflowModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
