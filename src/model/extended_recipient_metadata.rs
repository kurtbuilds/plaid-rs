use serde::{Serialize, Deserialize};
use super::FdxRecipientMetadata;
///Plaid and FDX-defined recipient metadata fields
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtendedRecipientMetadata {
    ///Recipient metadata fields that are defined by FDX
    #[serde(flatten)]
    pub fdx_recipient_metadata: FdxRecipientMetadata,
    ///The category that the recipient falls under
    pub category: String,
    ///The number of Data Partner consumers that are connected to the recipient for the specific Data Partner
    pub connection_count: i64,
    ///The date at which the recipient gained production access to Plaid
    pub joined_date: chrono::NaiveDate,
}
impl std::fmt::Display for ExtendedRecipientMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ExtendedRecipientMetadata {
    type Target = FdxRecipientMetadata;
    fn deref(&self) -> &Self::Target {
        &self.fdx_recipient_metadata
    }
}
impl std::ops::DerefMut for ExtendedRecipientMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fdx_recipient_metadata
    }
}
