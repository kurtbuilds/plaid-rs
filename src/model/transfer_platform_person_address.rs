use serde::{Serialize, Deserialize};
///Home address of a person
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferPlatformPersonAddress {
    ///The full city name.
    pub city: String,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
    ///The postal code of the address.
    pub postal_code: String,
    /**An ISO 3166-2 subdivision code.
Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.*/
    pub region: String,
    ///The primary street portion of an address. A string with at least one non-whitespace alphabetical character, with a max length of 80 characters.
    pub street: String,
    ///Extra street information, like an apartment or suite number. If provided, a string with at least one non-whitespace character, with a max length of 50 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
}
impl std::fmt::Display for TransferPlatformPersonAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
