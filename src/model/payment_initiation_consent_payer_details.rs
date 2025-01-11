use serde::{Serialize, Deserialize};
use super::{PaymentInitiationAddress, PaymentInitiationConsentPayerNumbers};
/**An object representing the payment consent payer details.
Payer `name` and account `numbers` are required to lock the account to which the consent can be created.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentInitiationConsentPayerDetails {
    ///The optional address of the payment recipient's bank account. Required by most institutions outside of the UK.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PaymentInitiationAddress>,
    ///The payer's birthdate, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    ///The payer's emails
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    ///The name of the payer as it appears in their bank account
    pub name: String,
    ///The counterparty's bank account numbers. Exactly one of IBAN or BACS data is required.
    pub numbers: PaymentInitiationConsentPayerNumbers,
    ///The payer's phone numbers in E.164 format: +{countrycode}{number}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
}
impl std::fmt::Display for PaymentInitiationConsentPayerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
