use serde::{Serialize, Deserialize};
use super::PaymentScheduleInterval;
///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleBase {
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, this field will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjusted_start_date: Option<chrono::NaiveDate>,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will end on the last `interval_execution_day` on or before the `end_date`.
If the only `interval_execution_day` between the start date and the end date (inclusive) is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make a payment on that day, but it is not guaranteed to do so.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    ///The frequency interval of the payment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<PaymentScheduleInterval>,
    /**The day of the interval on which to schedule the payment.

If the payment interval is weekly, `interval_execution_day` should be an integer from 1 (Monday) to 7 (Sunday).

If the payment interval is monthly, `interval_execution_day` should be an integer indicating which day of the month to make the payment on. Integers from 1 to 28 can be used to make a payment on that day of the month. Negative integers from -1 to -5 can be used to make a payment relative to the end of the month. To make a payment on the last day of the month, use -1; to make the payment on the second-to-last day, use -2, and so on.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_execution_day: Option<i64>,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will begin on the first `interval_execution_day` on or after the `start_date`.

If the first `interval_execution_day` on or after the start date is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make the first payment on that day, but it is not guaranteed to do so.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for ExternalPaymentScheduleBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
