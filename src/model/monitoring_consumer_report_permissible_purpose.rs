use serde::{Serialize, Deserialize};
/**Describes the reason you are generating a Consumer Report for this user.

`ACCOUNT_REVIEW_CREDIT`: In connection with a consumer credit transaction for the review or collection of an account pursuant to FCRA Section 604(a)(3)(A).

`WRITTEN_INSTRUCTION_OTHER`: In accordance with the written instructions of the consumer pursuant to FCRA Section 604(a)(2), such as when an individual agrees to act as a guarantor or assumes personal liability for a consumer, business, or commercial loan.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MonitoringConsumerReportPermissiblePurpose {
    #[serde(rename = "ACCOUNT_REVIEW_CREDIT")]
    AccountReviewCredit,
    #[serde(rename = "WRITTEN_INSTRUCTION_OTHER")]
    WrittenInstructionOther,
}
