use serde::{Serialize, Deserialize};
///An optional object to filter `/beacon/account_risk/v1/evaluate` results to a subset of the accounts on the linked Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconAccountRiskEvaluateRequestOptions {
    ///An array of `account_ids` for the specific accounts to evaluate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BeaconAccountRiskEvaluateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
