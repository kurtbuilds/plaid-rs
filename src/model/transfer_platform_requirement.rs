use serde::{Serialize, Deserialize};
///A piece of information that is outstanding for the scaled platform onboarding process.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferPlatformRequirement {
    ///UUID of the person associated with the requirement. Only present for individual-scoped requirements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    ///The type of requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirement_type: Option<String>,
}
impl std::fmt::Display for TransferPlatformRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
