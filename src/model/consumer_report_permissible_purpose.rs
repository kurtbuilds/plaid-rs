use serde::{Serialize, Deserialize};
/**Describes the reason you are generating a Consumer Report for this user.

`ACCOUNT_REVIEW_CREDIT`: In connection with a consumer credit transaction for the review or collection of an account pursuant to FCRA Section 604(a)(3)(A).

`ACCOUNT_REVIEW_NON_CREDIT`: For a legitimate business need of the information to review a non-credit account provided primarily for personal, family, or household purposes to determine whether the consumer continues to meet the terms of the account pursuant to FCRA Section 604(a)(3)(F)(2).

`EMPLOYMENT`: For employment purposes pursuant to FCRA 604(a)(3)(B), including hiring, retention and promotion purposes.

`EXTENSION_OF_CREDIT`: In connection with a credit transaction initiated by and involving the consumer pursuant to FCRA Section 604(a)(3)(A).

`LEGITIMATE_BUSINESS_NEED_TENANT_SCREENING`: For a legitimate business need in connection with a business transaction initiated by the consumer primarily for personal, family, or household purposes in connection with a property rental assessment pursuant to FCRA Section 604(a)(3)(F)(i).

`LEGITIMATE_BUSINESS_NEED_OTHER`: For a legitimate business need in connection with a business transaction made primarily for personal, family, or household initiated by the consumer pursuant to FCRA Section 604(a)(3)(F)(i).

`WRITTEN_INSTRUCTION_PREQUALIFICATION`: In accordance with the written instructions of the consumer pursuant to FCRA Section 604(a)(2), to evaluate an application’s profile to make an offer to the consumer.

`WRITTEN_INSTRUCTION_OTHER`: In accordance with the written instructions of the consumer pursuant to FCRA Section 604(a)(2), such as when an individual agrees to act as a guarantor or assumes personal liability for a consumer, business, or commercial loan.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConsumerReportPermissiblePurpose {
    #[serde(rename = "ACCOUNT_REVIEW_CREDIT")]
    AccountReviewCredit,
    #[serde(rename = "ACCOUNT_REVIEW_NON_CREDIT")]
    AccountReviewNonCredit,
    #[serde(rename = "EMPLOYMENT")]
    Employment,
    #[serde(rename = "EXTENSION_OF_CREDIT")]
    ExtensionOfCredit,
    #[serde(rename = "LEGITIMATE_BUSINESS_NEED_TENANT_SCREENING")]
    LegitimateBusinessNeedTenantScreening,
    #[serde(rename = "LEGITIMATE_BUSINESS_NEED_OTHER")]
    LegitimateBusinessNeedOther,
    #[serde(rename = "WRITTEN_INSTRUCTION_PREQUALIFICATION")]
    WrittenInstructionPrequalification,
    #[serde(rename = "WRITTEN_INSTRUCTION_OTHER")]
    WrittenInstructionOther,
}
