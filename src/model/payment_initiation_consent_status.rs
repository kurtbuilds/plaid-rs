use serde::{Serialize, Deserialize};
/**The status of the payment consent.

`UNAUTHORISED`: Consent created, but requires user authorisation.

`REJECTED`: Consent authorisation was rejected by the user and/or the bank.

`AUTHORISED`: Consent is active and ready to be used.

`REVOKED`: Consent has been revoked and can no longer be used.

`EXPIRED`: Consent is no longer valid.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentInitiationConsentStatus {
    #[serde(rename = "UNAUTHORISED")]
    Unauthorised,
    #[serde(rename = "AUTHORISED")]
    Authorised,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "EXPIRED")]
    Expired,
}
