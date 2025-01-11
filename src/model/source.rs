use serde::{Serialize, Deserialize};
///A type indicating whether a dashboard user, an API-based user, or Plaid last touched this object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Source {
    #[serde(rename = "dashboard")]
    Dashboard,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "system")]
    System,
}
