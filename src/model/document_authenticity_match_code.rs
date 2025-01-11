use serde::{Serialize, Deserialize};
/**High level summary of whether the document in the provided image matches the formatting rules and security checks for the associated jurisdiction.

For example, most identity documents have formatting rules like the following:


The image of the person's face must have a certain contrast in order to highlight skin tone


The subject in the document's image must remove eye glasses and pose in a certain way


The informational fields (name, date of birth, ID number, etc.) must be colored and aligned according to specific rules


Security features like watermarks and background patterns must be present

So a `match` status for this field indicates that the document in the provided image seems to conform to the various formatting and security rules associated with the detected document.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DocumentAuthenticityMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,
}
