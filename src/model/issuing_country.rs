use serde::{Serialize, Deserialize};
/**A binary match indicator specifying whether the country that issued the provided document matches the country that the user separately provided to Plaid.

Note: You can configure whether a `no_match` on `issuing_country` fails the `documentary_verification` by editing your Plaid Template.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IssuingCountry {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "no_match")]
    NoMatch,
}
