use serde::{Serialize, Deserialize};
use super::{
    PaymentConsentPeriodicAlignment, PaymentConsentPeriodicAmountAmount,
    PaymentConsentPeriodicInterval,
};
///Defines consent payments limitations per period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmount {
    /**Where the payment consent period should start.

If the institution is Monzo, only `CONSENT` alignments are supported.

`CALENDAR`: line up with a calendar.

`CONSENT`: on the date of consent creation.*/
    pub alignment: PaymentConsentPeriodicAlignment,
    ///Maximum cumulative amount for all payments in the specified interval.
    pub amount: PaymentConsentPeriodicAmountAmount,
    ///Payment consent periodic interval.
    pub interval: PaymentConsentPeriodicInterval,
}
impl std::fmt::Display for PaymentConsentPeriodicAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
