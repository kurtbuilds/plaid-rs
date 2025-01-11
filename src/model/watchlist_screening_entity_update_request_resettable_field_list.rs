use serde::{Serialize, Deserialize};
use super::WatchlistScreeningEntityUpdateRequestResettableField;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistScreeningEntityUpdateRequestResettableFieldList(
    pub Vec<WatchlistScreeningEntityUpdateRequestResettableField>,
);
