use serde::{Serialize, Deserialize};
///ISO-3166-1 alpha-2 country code standard.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CountryCode {
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "BE")]
    Be,
}
