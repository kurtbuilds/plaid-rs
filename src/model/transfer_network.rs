use serde::{Serialize, Deserialize};
/**The network or rails used for the transfer.

For transfers submitted as `ach`, the next-day cutoff is 8:30 PM Eastern Time.

For transfers submitted as `same-day-ach`, the same-day cutoff is 3:30 PM Eastern Time. If the transfer is submitted after this cutoff but before the next-day cutoff, it will be sent over next-day rails and will not incur same-day charges; this will apply to both legs of the transfer if applicable.

For transfers submitted as `rtp`,  Plaid will automatically route between Real Time Payment rail by TCH or FedNow rails as necessary. If a transfer is submitted as `rtp` and the counterparty account is not eligible for RTP, the `/transfer/authorization/create` request will fail with an `INVALID_FIELD` error code. To pre-check to determine whether a counterparty account can support RTP, call `/transfer/capabilities/get` before calling `/transfer/authorization/create`.

Wire transfers are currently in early availability. To request access to `wire` as a payment network, contact your Account Manager. For transfers submitted as `wire`, the `type` must be `credit`; wire debits are not supported.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "rtp")]
    Rtp,
    #[serde(rename = "wire")]
    Wire,
}
