use serde::{Serialize, Deserialize};
///The user's first name and last name.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAccountIdentityName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}
impl std::fmt::Display for UserAccountIdentityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
