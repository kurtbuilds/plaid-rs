use serde::{Serialize, Deserialize};
/**The type of Beacon Report.

`first_party`: If this is the same individual as the one who submitted the KYC.

`stolen`: If this is a different individual from the one who submitted the KYC.

`synthetic`: If this is an individual using fabricated information.

`account_takeover`: If this individual's account was compromised.

`unknown`: If you aren't sure who committed the fraud.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BeaconReportCreateType {
    #[serde(rename = "first_party")]
    FirstParty,
    #[serde(rename = "stolen")]
    Stolen,
    #[serde(rename = "synthetic")]
    Synthetic,
    #[serde(rename = "account_takeover")]
    AccountTakeover,
    #[serde(rename = "data_breach")]
    DataBreach,
    #[serde(rename = "unknown")]
    Unknown,
}
