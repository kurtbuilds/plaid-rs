use serde::{Serialize, Deserialize};
/**Specifies the use case of the transfer. Required for transfers on an ACH network. For more details, see [ACH SEC codes](https://plaid.com/docs/transfer/creating-transfers/#ach-sec-codes).

Codes supported for credits: `ccd`, `ppd`
Codes supported for debits: `ccd`, `tel`, `web`

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, e.g. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AchClass {
    #[serde(rename = "ccd")]
    Ccd,
    #[serde(rename = "ppd")]
    Ppd,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "web")]
    Web,
}
