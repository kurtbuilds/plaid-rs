use serde::{Serialize, Deserialize};
use super::PaymentInitiationConsentStatus;
///PaymentInitiationConsentCreateResponse defines the response schema for `/payment_initiation/consent/create`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateResponse {
    ///A unique ID identifying the payment consent.
    pub consent_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /**The status of the payment consent.

`UNAUTHORISED`: Consent created, but requires user authorisation.

`REJECTED`: Consent authorisation was rejected by the user and/or the bank.

`AUTHORISED`: Consent is active and ready to be used.

`REVOKED`: Consent has been revoked and can no longer be used.

`EXPIRED`: Consent is no longer valid.*/
    pub status: PaymentInitiationConsentStatus,
}
impl std::fmt::Display for PaymentInitiationConsentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
