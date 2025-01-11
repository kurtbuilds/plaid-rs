use serde::{Serialize, Deserialize};
use super::BeaconBankAccounts;
///The response schema for `/beacon/user/account/insights/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserAccountInsightsGetResponse {
    ///A collection of Bank Accounts linked to an Item that is associated with this Beacon User.
    pub bank_account_insights: BeaconBankAccounts,
    ///ID of the associated Beacon User.
    pub beacon_user_id: String,
    ///An ISO8601 formatted timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///An ISO8601 formatted timestamp. This field indicates the last time the resource was modified.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for BeaconUserAccountInsightsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
