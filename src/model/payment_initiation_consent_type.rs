use serde::{Serialize, Deserialize};
/**Payment consent type. Defines possible use case for payments made with the given consent.

`SWEEPING`: Allows moving money between accounts owned by the same user.

`COMMERCIAL`: Allows initiating payments from the user's account to third parties.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentInitiationConsentType {
    #[serde(rename = "SWEEPING")]
    Sweeping,
    #[serde(rename = "COMMERCIAL")]
    Commercial,
}
