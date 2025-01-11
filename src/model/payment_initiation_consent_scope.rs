use serde::{Serialize, Deserialize};
/**This field is deprecated in favor of the consent `type` field. Consents are required to have a single type.

Payment consent scope. Defines possible directions for payments made with the given consent.

`ME_TO_ME`: Allows moving money between accounts owned by the same user.

`EXTERNAL`: Allows initiating payments from the user's account to third parties.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentInitiationConsentScope {
    #[serde(rename = "ME_TO_ME")]
    MeToMe,
    #[serde(rename = "EXTERNAL")]
    External,
}
