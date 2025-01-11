use serde::{Serialize, Deserialize};
///A map of network attributes, where the key is a string, and the value is a float, int, or boolean.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkInsightsSchema {}
impl std::fmt::Display for NetworkInsightsSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
