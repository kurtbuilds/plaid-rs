use serde::{Serialize, Deserialize};
/**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"web"` - A credit Entry initiated by or on behalf of a holder of a Consumer Account that is intended for a Consumer Account of a Receiver*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CreditAchClass {
    #[serde(rename = "ccd")]
    Ccd,
    #[serde(rename = "ppd")]
    Ppd,
    #[serde(rename = "web")]
    Web,
}
