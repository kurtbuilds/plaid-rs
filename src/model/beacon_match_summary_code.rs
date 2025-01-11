use serde::{Serialize, Deserialize};
/**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BeaconMatchSummaryCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,
}
