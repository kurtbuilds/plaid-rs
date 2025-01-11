use serde::{Serialize, Deserialize};
use super::ConsentEvent;
///Describes a historical log of item consent events.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsentEventsGetResponse {
    ///A list of consent events.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consent_events: Vec<ConsentEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ConsentEventsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
