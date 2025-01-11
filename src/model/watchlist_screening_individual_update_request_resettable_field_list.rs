use serde::{Serialize, Deserialize};
use super::WatchlistScreeningIndividualUpdateRequestResettableField;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningIndividualUpdateRequestResettableFieldList(
    pub Vec<WatchlistScreeningIndividualUpdateRequestResettableField>,
);
