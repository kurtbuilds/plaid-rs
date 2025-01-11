use serde::{Serialize, Deserialize};
use super::RecipientBacs;
///The counterparty's bank account numbers. Exactly one of IBAN or BACS data is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationConsentPayerNumbers {
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bacs: Option<RecipientBacs>,
    ///International Bank Account Number (IBAN).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
}
impl std::fmt::Display for PaymentInitiationConsentPayerNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
