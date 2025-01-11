use serde::{Serialize, Deserialize};
///ProfileIdentityName defines the user's first name and last name.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileIdentityName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}
impl std::fmt::Display for ProfileIdentityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
