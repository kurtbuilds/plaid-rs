use serde::{Serialize, Deserialize};
/**Field describing whether the associated address is being used for commercial or residential purposes.

Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AddressPurposeLabel {
    #[serde(rename = "residential")]
    Residential,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "no_data")]
    NoData,
}
