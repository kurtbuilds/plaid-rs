use serde::{Serialize, Deserialize};
use super::{ConsentEventCode, ConsentEventInitiator, ConsentEventType, ConsentedAccount};
///Describes a consent event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentEvent {
    ///An array containing the accounts associated with the Item for which authorizations are granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consented_accounts: Option<Vec<ConsentedAccount>>,
    ///A list of strings containing the full list of data scopes the end user has consented to for the Item. These correspond to consented products; see the [full mapping](/docs/link/data-transparency-messaging-migration-guide/#data-scopes-by-product) of data scopes and products.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consented_data_scopes: Option<Vec<String>>,
    /**A list of strings containing the full list of use cases the end user has consented to for the Item.

See the [full list](/docs/link/data-transparency-messaging-migration-guide/#updating-link-customizations) of use cases.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consented_use_cases: Option<Vec<String>>,
    ///The date and time when the consent event occurred, in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Codes describing the object of a consent event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_code: Option<ConsentEventCode>,
    ///A broad categorization of the consent event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<ConsentEventType>,
    ///The entity that initiated collection of consent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator: Option<ConsentEventInitiator>,
    ///Unique identifier for the institution associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The full name of the institution associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}
impl std::fmt::Display for ConsentEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
