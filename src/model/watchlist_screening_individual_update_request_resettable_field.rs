use serde::{Serialize, Deserialize};
///The name of a field that can be reset back to null
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WatchlistScreeningIndividualUpdateRequestResettableField {
    #[serde(rename = "assignee")]
    Assignee,
}
