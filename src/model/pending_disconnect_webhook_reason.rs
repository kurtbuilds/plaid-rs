use serde::{Serialize, Deserialize};
/**Reason why the item is about to be disconnected.
`INSTITUTION_MIGRATION`: The institution is moving to API or to a different integration. For example, this can occur when an institution moves from a non-OAuth integration to an OAuth integration.
`INSTITUTION_TOKEN_EXPIRATION`: The consent on an Item associated with a US or CA institution is about to expire.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PendingDisconnectWebhookReason {
    #[serde(rename = "INSTITUTION_MIGRATION")]
    InstitutionMigration,
    #[serde(rename = "INSTITUTION_TOKEN_EXPIRATION")]
    InstitutionTokenExpiration,
}
