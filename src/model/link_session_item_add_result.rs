use serde::{Serialize, Deserialize};
use super::{LinkSessionSuccessMetadataAccount, LinkSessionSuccessMetadataInstitution};
///The details of an Item add in Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkSessionItemAddResult {
    ///A list of accounts attached to the connected Item. If Account Select is enabled via the developer dashboard, `accounts` will only include selected accounts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<LinkSessionSuccessMetadataAccount>,
    ///An institution object. If the Item was created via Same-Day or Instant micro-deposit verification, will be `null`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution: Option<LinkSessionSuccessMetadataInstitution>,
    ///Returned once a user has successfully linked their Item.
    pub public_token: String,
}
impl std::fmt::Display for LinkSessionItemAddResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
