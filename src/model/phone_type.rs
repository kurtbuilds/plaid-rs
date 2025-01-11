use serde::{Serialize, Deserialize};
///An enum indicating whether a phone number is a phone line or a fax line.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PhoneType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "fax")]
    Fax,
}
