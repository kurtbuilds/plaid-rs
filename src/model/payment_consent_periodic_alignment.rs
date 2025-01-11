use serde::{Serialize, Deserialize};
/**Where the payment consent period should start.

If the institution is Monzo, only `CONSENT` alignments are supported.

`CALENDAR`: line up with a calendar.

`CONSENT`: on the date of consent creation.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentConsentPeriodicAlignment {
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "CONSENT")]
    Consent,
}
