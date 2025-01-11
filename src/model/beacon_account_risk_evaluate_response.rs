use serde::{Serialize, Deserialize};
use super::BeaconAccountRiskEvaluateAccount;
///BeaconAccountRiskEvaluateResponse defines the response schema for `/beacon/account_risk/v1/evaluate`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconAccountRiskEvaluateResponse {
    ///The accounts for which a risk evaluation has been requested.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<BeaconAccountRiskEvaluateAccount>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BeaconAccountRiskEvaluateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
