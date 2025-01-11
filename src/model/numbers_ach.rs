use serde::{Serialize, Deserialize};
///Identifying information for transferring money to or from a US account via ACH or wire transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NumbersAch {
    /**The ACH account number for the account.

At certain institutions, including Chase and PNC, you will receive "tokenized" routing and account numbers, which are not the user's actual account and routing numbers. For important details on how this may impact your integration and on how to avoid fraud, user confusion, and ACH returns, see [Tokenized account numbers](https://plaid.com/docs/auth/#tokenized-account-numbers).*/
    pub account: String,
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///Whether the account supports ACH transfers into the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_transfer_in: Option<bool>,
    ///Whether the account supports ACH transfers out of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_transfer_out: Option<bool>,
    ///The ACH routing number for the account. This may be a tokenized routing number. For more information, see [Tokenized account numbers](https://plaid.com/docs/auth/#tokenized-account-numbers).
    pub routing: String,
    ///The wire transfer routing number for the account, if available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wire_routing: Option<String>,
}
impl std::fmt::Display for NumbersAch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
