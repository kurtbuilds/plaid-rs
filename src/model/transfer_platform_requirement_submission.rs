use serde::{Serialize, Deserialize};
///A single requirement submission
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferPlatformRequirementSubmission {
    ///The `person_id` of the person the requirement submission is related to. A `person_id` is returned by `/transfer/platform/person/create`. This field should not be included for requirements that are not related to a person.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    ///The type of requirement being submitted
    pub requirement_type: String,
    ///The value of the requirement, which can be a string or an object depending on the `requirement_type`. If it is an object, the object should be JSON marshaled into a string. See the documentation on this endpoint for more information and examples.
    pub value: String,
}
impl std::fmt::Display for TransferPlatformRequirementSubmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
