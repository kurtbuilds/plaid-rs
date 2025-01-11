use serde::{Serialize, Deserialize};
/**The network or rails used for the transfer. Defaults to `same-day-ach`.

For transfers submitted using `ach`, the next-day cutoff is 8:30 PM Eastern Time.

For transfers submitted using `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges.

For transfers submitted using `rtp`, in the case that the account being credited does not support RTP, the transfer will be sent over ACH as long as an `ach_class` is provided in the request. If RTP isn't supported by the account and no `ach_class` is provided, the transfer will fail to be submitted.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferIntentCreateNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "rtp")]
    Rtp,
}
