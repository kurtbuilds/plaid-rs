use serde::{Serialize, Deserialize};
/**The direction of the flow of transfer funds.

`PAYMENT`: Transfers funds from an end user's account to your business account.

`DISBURSEMENT`: Transfers funds from your business account to an end user's account.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferIntentCreateMode {
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "DISBURSEMENT")]
    Disbursement,
}
