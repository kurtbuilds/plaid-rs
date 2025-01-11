use serde::{Serialize, Deserialize};
/**The kind of official document represented by this object.

`bik` - Russian bank code

`business_number` - A number that uniquely identifies the business within a category of businesses

`imo` - Number assigned to the entity by the International Maritime Organization

`other` - Any document not covered by other categories

`swift` - Number identifying a bank and branch.

`tax_id` - Identification issued for the purpose of collecting taxes*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityDocumentType {
    #[serde(rename = "bik")]
    Bik,
    #[serde(rename = "business_number")]
    BusinessNumber,
    #[serde(rename = "imo")]
    Imo,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "tax_id")]
    TaxId,
}
