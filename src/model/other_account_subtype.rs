use serde::{Serialize, Deserialize};
///Valid account subtypes for other accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-other).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OtherAccountSubtype {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "all")]
    All,
}
