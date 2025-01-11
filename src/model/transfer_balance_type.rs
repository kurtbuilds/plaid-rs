use serde::{Serialize, Deserialize};
/**The type of balance.

`prefunded_rtp_credits` - Your prefunded RTP credit balance with Plaid
`prefunded_ach_credits` - Your prefunded ACH credit balance with Plaid*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferBalanceType {
    #[serde(rename = "prefunded_rtp_credits")]
    PrefundedRtpCredits,
    #[serde(rename = "prefunded_ach_credits")]
    PrefundedAchCredits,
}
