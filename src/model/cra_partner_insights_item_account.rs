use serde::{Serialize, Deserialize};
use super::{
    CraPartnerInsightsItemAccountMetadata, CreditBankIncomeAccountType,
    DepositoryAccountSubtype, Owner,
};
///Account data corresponding to the item from which Partner Insights were generated from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraPartnerInsightsItemAccount {
    /**Plaid's unique identifier for the account. This value will not change unless Plaid can't reconcile the account with the data returned by the financial institution. This may occur, for example, when the name of the account changes. If this happens a new `account_id` will be assigned to the account.

If an account with a specific `account_id` disappears instead of changing, the account is likely closed. Closed accounts are not returned by the Plaid API.

Like all Plaid identifiers, the `account_id` is case sensitive.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /**The last 2-4 alphanumeric characters of an account's official account number.
Note that the mask may be non-unique between an Item's accounts, and it may also
not match the mask that the bank displays to the user.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    ///An object containing metadata about the extracted account.
    pub metadata: CraPartnerInsightsItemAccountMetadata,
    ///The name of the account
    pub name: String,
    ///The official name of the bank account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    ///Data returned by the financial institution about the account owner or owners. Identity information is optional, so field may return an empty array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<Owner>,
    ///Valid account subtypes for depository accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-depository).
    pub subtype: DepositoryAccountSubtype,
    ///The account type. This will always be `depository`.
    #[serde(rename = "type")]
    pub type_: CreditBankIncomeAccountType,
}
impl std::fmt::Display for CraPartnerInsightsItemAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
